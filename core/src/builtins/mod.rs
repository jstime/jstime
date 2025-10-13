pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(printer),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(queue_microtask),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(temporal_now),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(temporal_plain_date),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(temporal_plain_time),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(temporal_plain_date_time),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(temporal_instant),
        },
    ]
}

pub(crate) struct Builtins {}

impl Builtins {
    pub(crate) fn create(scope: &mut v8::HandleScope) {
        let bindings = v8::Object::new(scope);

        macro_rules! binding {
            ($name:expr, $fn:ident) => {
                let name = v8::String::new(scope, $name).unwrap();
                let value = v8::Function::new(scope, $fn).unwrap();
                bindings.set(scope, name.into(), value.into());
            };
        }

        binding!("printer", printer);
        binding!("queueMicrotask", queue_microtask);
        binding!("temporalNow", temporal_now);
        binding!("temporalPlainDate", temporal_plain_date);
        binding!("temporalPlainTime", temporal_plain_time);
        binding!("temporalPlainDateTime", temporal_plain_date_time);
        binding!("temporalInstant", temporal_instant);

        macro_rules! builtin {
            ($name:expr) => {
                let source = include_str!($name);
                let val = match crate::script::run(scope, source, $name) {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                };
                let func = v8::Local::<v8::Function>::try_from(val).unwrap();
                let recv = v8::undefined(scope).into();
                let args = [bindings.into()];
                func.call(scope, recv, &args).unwrap();
            };
        }

        builtin!("./console.js");
        builtin!("./queue_microtask.js");
        builtin!("./temporal.js");
    }
}

fn printer(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    let arg_len = args.length();
    assert!((0..=2).contains(&arg_len));

    let obj = args.get(0);
    let is_err_arg = args.get(1);

    let mut is_err = false;
    if arg_len == 2 {
        let int_val = is_err_arg
            .integer_value(scope)
            .expect("Unable to convert to integer");
        is_err = int_val != 0;
    };
    let tc_scope = &mut v8::TryCatch::new(scope);
    let str_ = match obj.to_string(tc_scope) {
        Some(s) => s,
        None => v8::String::new(tc_scope, "").unwrap(),
    };
    if is_err {
        eprintln!("{}", str_.to_rust_string_lossy(tc_scope));
    } else {
        println!("{}", str_.to_rust_string_lossy(tc_scope));
    }
}

fn queue_microtask(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let obj = args.get(0);
    let func = v8::Local::<v8::Function>::try_from(obj).unwrap();
    scope.enqueue_microtask(func);
}

// Temporal API implementations
fn temporal_now(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    use temporal_rs::Temporal;

    let method_arg = args.get(0);
    let method_str = method_arg.to_string(scope).unwrap();
    let method = method_str.to_rust_string_lossy(scope);

    let now = Temporal::now();
    let result = match method.as_str() {
        "instant" => {
            match now.instant() {
                Ok(instant) => {
                    let ns = instant.as_i128();
                    let obj = v8::Object::new(scope);
                    let key = v8::String::new(scope, "epochNanoseconds").unwrap();
                    let value = v8::BigInt::new_from_i64(scope, ns as i64);
                    obj.set(scope, key.into(), value.into());
                    Some(obj.into())
                }
                Err(_) => None,
            }
        }
        "plainDateISO" => {
            match now.plain_date_iso(None) {
                Ok(date) => {
                    let obj = v8::Object::new(scope);
                    set_date_fields(scope, &obj, date.year(), date.month() as i32, date.day() as i32);
                    Some(obj.into())
                }
                Err(_) => None,
            }
        }
        "plainTimeISO" => {
            match now.plain_time_iso(None) {
                Ok(time) => {
                    let obj = v8::Object::new(scope);
                    set_time_fields(scope, &obj, time.hour() as i32, time.minute() as i32, time.second() as i32, time.millisecond() as i32, time.microsecond() as i32, time.nanosecond() as i32);
                    Some(obj.into())
                }
                Err(_) => None,
            }
        }
        "plainDateTimeISO" => {
            match now.plain_date_time_iso(None) {
                Ok(datetime) => {
                    let obj = v8::Object::new(scope);
                    set_date_fields(scope, &obj, datetime.year(), datetime.month() as i32, datetime.day() as i32);
                    set_time_fields(scope, &obj, datetime.hour() as i32, datetime.minute() as i32, datetime.second() as i32, datetime.millisecond() as i32, datetime.microsecond() as i32, datetime.nanosecond() as i32);
                    Some(obj.into())
                }
                Err(_) => None,
            }
        }
        "zonedDateTimeISO" => {
            match now.zoned_date_time_iso(None) {
                Ok(zdt) => {
                    let obj = v8::Object::new(scope);
                    set_date_fields(scope, &obj, zdt.year(), zdt.month() as i32, zdt.day() as i32);
                    set_time_fields(scope, &obj, zdt.hour() as i32, zdt.minute() as i32, zdt.second() as i32, zdt.millisecond() as i32, zdt.microsecond() as i32, zdt.nanosecond() as i32);
                    Some(obj.into())
                }
                Err(_) => None,
            }
        }
        _ => None,
    };

    if let Some(val) = result {
        rv.set(val);
    }
}

fn temporal_plain_date(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    use temporal_rs::PlainDate;

    let year = args.get(0).int32_value(scope).unwrap_or(0);
    let month = args.get(1).int32_value(scope).unwrap_or(1);
    let day = args.get(2).int32_value(scope).unwrap_or(1);

    match PlainDate::try_new_iso(year, month as u8, day as u8) {
        Ok(date) => {
            let obj = v8::Object::new(scope);
            set_date_fields(scope, &obj, date.year(), date.month() as i32, date.day() as i32);
            rv.set(obj.into());
        }
        Err(_) => {
            let msg = v8::String::new(scope, "Invalid PlainDate").unwrap();
            let exception = v8::Exception::range_error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn temporal_plain_time(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    use temporal_rs::PlainTime;

    let hour = args.get(0).int32_value(scope).unwrap_or(0) as u8;
    let minute = args.get(1).int32_value(scope).unwrap_or(0) as u8;
    let second = args.get(2).int32_value(scope).unwrap_or(0) as u8;
    let millisecond = args.get(3).int32_value(scope).unwrap_or(0) as u16;
    let microsecond = args.get(4).int32_value(scope).unwrap_or(0) as u16;
    let nanosecond = args.get(5).int32_value(scope).unwrap_or(0) as u16;

    match PlainTime::try_new(hour, minute, second, millisecond, microsecond, nanosecond) {
        Ok(time) => {
            let obj = v8::Object::new(scope);
            set_time_fields(scope, &obj, time.hour() as i32, time.minute() as i32, time.second() as i32, time.millisecond() as i32, time.microsecond() as i32, time.nanosecond() as i32);
            rv.set(obj.into());
        }
        Err(_) => {
            let msg = v8::String::new(scope, "Invalid PlainTime").unwrap();
            let exception = v8::Exception::range_error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn temporal_plain_date_time(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    use temporal_rs::PlainDateTime;

    let year = args.get(0).int32_value(scope).unwrap_or(0);
    let month = args.get(1).int32_value(scope).unwrap_or(1) as u8;
    let day = args.get(2).int32_value(scope).unwrap_or(1) as u8;
    let hour = args.get(3).int32_value(scope).unwrap_or(0) as u8;
    let minute = args.get(4).int32_value(scope).unwrap_or(0) as u8;
    let second = args.get(5).int32_value(scope).unwrap_or(0) as u8;
    let millisecond = args.get(6).int32_value(scope).unwrap_or(0) as u16;
    let microsecond = args.get(7).int32_value(scope).unwrap_or(0) as u16;
    let nanosecond = args.get(8).int32_value(scope).unwrap_or(0) as u16;

    match PlainDateTime::try_new_iso(year, month, day, hour, minute, second, millisecond, microsecond, nanosecond) {
        Ok(datetime) => {
            let obj = v8::Object::new(scope);
            set_date_fields(scope, &obj, datetime.year(), datetime.month() as i32, datetime.day() as i32);
            set_time_fields(scope, &obj, datetime.hour() as i32, datetime.minute() as i32, datetime.second() as i32, datetime.millisecond() as i32, datetime.microsecond() as i32, datetime.nanosecond() as i32);
            rv.set(obj.into());
        }
        Err(_) => {
            let msg = v8::String::new(scope, "Invalid PlainDateTime").unwrap();
            let exception = v8::Exception::range_error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

fn temporal_instant(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    use temporal_rs::Instant;

    let item = args.get(0);
    let item_str = item.to_string(scope).unwrap();
    let item_string = item_str.to_rust_string_lossy(scope);

    match Instant::from_utf8(item_string.as_bytes()) {
        Ok(instant) => {
            let obj = v8::Object::new(scope);
            let key = v8::String::new(scope, "epochNanoseconds").unwrap();
            let value = v8::BigInt::new_from_i64(scope, instant.as_i128() as i64);
            obj.set(scope, key.into(), value.into());
            rv.set(obj.into());
        }
        Err(_) => {
            let msg = v8::String::new(scope, "Invalid Instant string").unwrap();
            let exception = v8::Exception::range_error(scope, msg);
            scope.throw_exception(exception);
        }
    }
}

// Helper functions to set object fields
fn set_date_fields(scope: &mut v8::HandleScope, obj: &v8::Local<v8::Object>, year: i32, month: i32, day: i32) {
    let year_key = v8::String::new(scope, "year").unwrap();
    let year_val = v8::Integer::new(scope, year);
    obj.set(scope, year_key.into(), year_val.into());

    let month_key = v8::String::new(scope, "month").unwrap();
    let month_val = v8::Integer::new(scope, month);
    obj.set(scope, month_key.into(), month_val.into());

    let day_key = v8::String::new(scope, "day").unwrap();
    let day_val = v8::Integer::new(scope, day);
    obj.set(scope, day_key.into(), day_val.into());
}

fn set_time_fields(scope: &mut v8::HandleScope, obj: &v8::Local<v8::Object>, hour: i32, minute: i32, second: i32, millisecond: i32, microsecond: i32, nanosecond: i32) {
    let hour_key = v8::String::new(scope, "hour").unwrap();
    let hour_val = v8::Integer::new(scope, hour);
    obj.set(scope, hour_key.into(), hour_val.into());

    let minute_key = v8::String::new(scope, "minute").unwrap();
    let minute_val = v8::Integer::new(scope, minute);
    obj.set(scope, minute_key.into(), minute_val.into());

    let second_key = v8::String::new(scope, "second").unwrap();
    let second_val = v8::Integer::new(scope, second);
    obj.set(scope, second_key.into(), second_val.into());

    let millisecond_key = v8::String::new(scope, "millisecond").unwrap();
    let millisecond_val = v8::Integer::new(scope, millisecond);
    obj.set(scope, millisecond_key.into(), millisecond_val.into());

    let microsecond_key = v8::String::new(scope, "microsecond").unwrap();
    let microsecond_val = v8::Integer::new(scope, microsecond);
    obj.set(scope, microsecond_key.into(), microsecond_val.into());

    let nanosecond_key = v8::String::new(scope, "nanosecond").unwrap();
    let nanosecond_val = v8::Integer::new(scope, nanosecond);
    obj.set(scope, nanosecond_key.into(), nanosecond_val.into());
}
