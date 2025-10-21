use v8::ValueDeserializerHelper;
use v8::ValueSerializerHelper;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![v8::ExternalReference {
        function: v8::MapFnTo::map_fn_to(structured_clone),
    }]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "structuredClone").unwrap();
    let value = v8::Function::new(scope, structured_clone).unwrap();
    bindings.set(scope, name.into(), value.into());
}

fn structured_clone(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Get the value to clone (first argument)
    let value = args.get(0);

    // Get the current context
    let context = scope.get_current_context();

    // Create a ValueSerializer to serialize the value
    let value_serializer = v8::ValueSerializer::new(scope, Box::new(StructuredCloneDelegate));
    value_serializer.write_header();

    if value_serializer.write_value(context, value).is_none() {
        // Serialization failed, throw an error
        let message = v8::String::new(scope, "Value could not be cloned").unwrap();
        let exception = v8::Exception::error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    let serialized = value_serializer.release();

    // Create a ValueDeserializer to deserialize the value
    let value_deserializer =
        v8::ValueDeserializer::new(scope, Box::new(StructuredCloneDelegate), &serialized);

    if value_deserializer.read_header(context).is_none() {
        // Failed to read header
        let message = v8::String::new(scope, "Failed to deserialize value").unwrap();
        let exception = v8::Exception::error(scope, message);
        scope.throw_exception(exception);
        return;
    }

    match value_deserializer.read_value(context) {
        Some(cloned_value) => {
            rv.set(cloned_value);
        }
        None => {
            // Deserialization failed
            let message = v8::String::new(scope, "Failed to deserialize value").unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}

struct StructuredCloneDelegate;

impl v8::ValueSerializerImpl for StructuredCloneDelegate {
    fn throw_data_clone_error<'s>(
        &self,
        scope: &mut v8::PinScope<'s, '_>,
        message: v8::Local<'s, v8::String>,
    ) {
        let exception = v8::Exception::error(scope, message);
        scope.throw_exception(exception);
    }
}

impl v8::ValueDeserializerImpl for StructuredCloneDelegate {
    fn read_host_object<'s>(
        &self,
        scope: &mut v8::PinScope<'s, '_>,
        _value_deserializer: &dyn v8::ValueDeserializerHelper,
    ) -> Option<v8::Local<'s, v8::Object>> {
        let msg =
            v8::String::new(scope, "Host objects are not supported in structuredClone").unwrap();
        let exc = v8::Exception::error(scope, msg);
        scope.throw_exception(exc);
        None
    }
}
