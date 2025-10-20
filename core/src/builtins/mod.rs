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
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_parse),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_href),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_origin),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_protocol),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_username),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_password),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_host),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_hostname),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_port),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_pathname),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_search),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_get_hash),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_href),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_protocol),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_username),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_password),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_host),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_hostname),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_port),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_pathname),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_search),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_set_hash),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_to_json),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_search_params_new),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_search_params_to_string),
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
        binding!("urlParse", url_parse);
        binding!("urlGetHref", url_get_href);
        binding!("urlGetOrigin", url_get_origin);
        binding!("urlGetProtocol", url_get_protocol);
        binding!("urlGetUsername", url_get_username);
        binding!("urlGetPassword", url_get_password);
        binding!("urlGetHost", url_get_host);
        binding!("urlGetHostname", url_get_hostname);
        binding!("urlGetPort", url_get_port);
        binding!("urlGetPathname", url_get_pathname);
        binding!("urlGetSearch", url_get_search);
        binding!("urlGetHash", url_get_hash);
        binding!("urlSetHref", url_set_href);
        binding!("urlSetProtocol", url_set_protocol);
        binding!("urlSetUsername", url_set_username);
        binding!("urlSetPassword", url_set_password);
        binding!("urlSetHost", url_set_host);
        binding!("urlSetHostname", url_set_hostname);
        binding!("urlSetPort", url_set_port);
        binding!("urlSetPathname", url_set_pathname);
        binding!("urlSetSearch", url_set_search);
        binding!("urlSetHash", url_set_hash);
        binding!("urlToJson", url_to_json);
        binding!("urlSearchParamsNew", url_search_params_new);
        binding!("urlSearchParamsToString", url_search_params_to_string);

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
        builtin!("./url.js");
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
        "instant" => match now.instant() {
            Ok(instant) => {
                let ns = instant.as_i128();
                let obj = v8::Object::new(scope);
                let key = v8::String::new(scope, "epochNanoseconds").unwrap();
                let value = v8::BigInt::new_from_i64(scope, ns as i64);
                obj.set(scope, key.into(), value.into());
                Some(obj.into())
            }
            Err(_) => None,
        },
        "plainDateISO" => match now.plain_date_iso(None) {
            Ok(date) => {
                let obj = v8::Object::new(scope);
                set_date_fields(
                    scope,
                    &obj,
                    date.year(),
                    date.month() as i32,
                    date.day() as i32,
                );
                Some(obj.into())
            }
            Err(_) => None,
        },
        "plainTimeISO" => match now.plain_time_iso(None) {
            Ok(time) => {
                let obj = v8::Object::new(scope);
                set_time_fields(
                    scope,
                    &obj,
                    TimeFields {
                        hour: time.hour() as i32,
                        minute: time.minute() as i32,
                        second: time.second() as i32,
                        millisecond: time.millisecond() as i32,
                        microsecond: time.microsecond() as i32,
                        nanosecond: time.nanosecond() as i32,
                    },
                );
                Some(obj.into())
            }
            Err(_) => None,
        },
        "plainDateTimeISO" => match now.plain_date_time_iso(None) {
            Ok(datetime) => {
                let obj = v8::Object::new(scope);
                set_date_fields(
                    scope,
                    &obj,
                    datetime.year(),
                    datetime.month() as i32,
                    datetime.day() as i32,
                );
                set_time_fields(
                    scope,
                    &obj,
                    TimeFields {
                        hour: datetime.hour() as i32,
                        minute: datetime.minute() as i32,
                        second: datetime.second() as i32,
                        millisecond: datetime.millisecond() as i32,
                        microsecond: datetime.microsecond() as i32,
                        nanosecond: datetime.nanosecond() as i32,
                    },
                );
                Some(obj.into())
            }
            Err(_) => None,
        },
        "zonedDateTimeISO" => match now.zoned_date_time_iso(None) {
            Ok(zdt) => {
                let obj = v8::Object::new(scope);
                set_date_fields(
                    scope,
                    &obj,
                    zdt.year(),
                    zdt.month() as i32,
                    zdt.day() as i32,
                );
                set_time_fields(
                    scope,
                    &obj,
                    TimeFields {
                        hour: zdt.hour() as i32,
                        minute: zdt.minute() as i32,
                        second: zdt.second() as i32,
                        millisecond: zdt.millisecond() as i32,
                        microsecond: zdt.microsecond() as i32,
                        nanosecond: zdt.nanosecond() as i32,
                    },
                );
                Some(obj.into())
            }
            Err(_) => None,
        },
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
            set_date_fields(
                scope,
                &obj,
                date.year(),
                date.month() as i32,
                date.day() as i32,
            );
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
            set_time_fields(
                scope,
                &obj,
                TimeFields {
                    hour: time.hour() as i32,
                    minute: time.minute() as i32,
                    second: time.second() as i32,
                    millisecond: time.millisecond() as i32,
                    microsecond: time.microsecond() as i32,
                    nanosecond: time.nanosecond() as i32,
                },
            );
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

    match PlainDateTime::try_new_iso(
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
        microsecond,
        nanosecond,
    ) {
        Ok(datetime) => {
            let obj = v8::Object::new(scope);
            set_date_fields(
                scope,
                &obj,
                datetime.year(),
                datetime.month() as i32,
                datetime.day() as i32,
            );
            set_time_fields(
                scope,
                &obj,
                TimeFields {
                    hour: datetime.hour() as i32,
                    minute: datetime.minute() as i32,
                    second: datetime.second() as i32,
                    millisecond: datetime.millisecond() as i32,
                    microsecond: datetime.microsecond() as i32,
                    nanosecond: datetime.nanosecond() as i32,
                },
            );
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
fn set_date_fields(
    scope: &mut v8::HandleScope,
    obj: &v8::Local<v8::Object>,
    year: i32,
    month: i32,
    day: i32,
) {
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

struct TimeFields {
    hour: i32,
    minute: i32,
    second: i32,
    millisecond: i32,
    microsecond: i32,
    nanosecond: i32,
}

fn set_time_fields(scope: &mut v8::HandleScope, obj: &v8::Local<v8::Object>, fields: TimeFields) {
    let hour_key = v8::String::new(scope, "hour").unwrap();
    let hour_val = v8::Integer::new(scope, fields.hour);
    obj.set(scope, hour_key.into(), hour_val.into());

    let minute_key = v8::String::new(scope, "minute").unwrap();
    let minute_val = v8::Integer::new(scope, fields.minute);
    obj.set(scope, minute_key.into(), minute_val.into());

    let second_key = v8::String::new(scope, "second").unwrap();
    let second_val = v8::Integer::new(scope, fields.second);
    obj.set(scope, second_key.into(), second_val.into());

    let millisecond_key = v8::String::new(scope, "millisecond").unwrap();
    let millisecond_val = v8::Integer::new(scope, fields.millisecond);
    obj.set(scope, millisecond_key.into(), millisecond_val.into());

    let microsecond_key = v8::String::new(scope, "microsecond").unwrap();
    let microsecond_val = v8::Integer::new(scope, fields.microsecond);
    obj.set(scope, microsecond_key.into(), microsecond_val.into());

    let nanosecond_key = v8::String::new(scope, "nanosecond").unwrap();
    let nanosecond_val = v8::Integer::new(scope, fields.nanosecond);
    obj.set(scope, nanosecond_key.into(), nanosecond_val.into());
}

use url::Url;

// Helper to convert v8 string to Rust string
fn to_rust_string(scope: &mut v8::HandleScope, value: v8::Local<v8::Value>) -> String {
    value.to_string(scope).unwrap().to_rust_string_lossy(scope)
}

// Helper to create v8 string from Rust string
fn to_v8_string<'a>(scope: &mut v8::HandleScope<'a>, s: &str) -> v8::Local<'a, v8::String> {
    v8::String::new(scope, s).unwrap()
}

// URL parsing function
fn url_parse(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));

    let parsed = if args.length() > 1 && !args.get(1).is_undefined() {
        let base_str = to_rust_string(scope, args.get(1));
        match Url::parse(&base_str) {
            Ok(base) => base.join(&url_str),
            Err(_) => {
                rv.set(v8::null(scope).into());
                return;
            }
        }
    } else {
        Url::parse(&url_str)
    };

    match parsed {
        Ok(url) => {
            let url_str = url.to_string();
            let v8_str = to_v8_string(scope, &url_str);
            rv.set(v8_str.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_get_href(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_get_origin(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let origin = url.origin().ascii_serialization();
        let v8_str = to_v8_string(scope, &origin);
        rv.set(v8_str.into());
    }
}

fn url_get_protocol(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let protocol = format!("{}:", url.scheme());
        let v8_str = to_v8_string(scope, &protocol);
        rv.set(v8_str.into());
    }
}

fn url_get_username(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let v8_str = to_v8_string(scope, url.username());
        rv.set(v8_str.into());
    }
}

fn url_get_password(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let password = url.password().unwrap_or("");
        let v8_str = to_v8_string(scope, password);
        rv.set(v8_str.into());
    }
}

fn url_get_host(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let host = url.host_str().unwrap_or("");
        let host_with_port = if let Some(port) = url.port() {
            format!("{}:{}", host, port)
        } else {
            host.to_string()
        };
        let v8_str = to_v8_string(scope, &host_with_port);
        rv.set(v8_str.into());
    }
}

fn url_get_hostname(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let hostname = url.host_str().unwrap_or("");
        let v8_str = to_v8_string(scope, hostname);
        rv.set(v8_str.into());
    }
}

fn url_get_port(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let port = url.port().map(|p| p.to_string()).unwrap_or_default();
        let v8_str = to_v8_string(scope, &port);
        rv.set(v8_str.into());
    }
}

fn url_get_pathname(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let v8_str = to_v8_string(scope, url.path());
        rv.set(v8_str.into());
    }
}

fn url_get_search(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let search = url.query().map(|q| format!("?{}", q)).unwrap_or_default();
        let v8_str = to_v8_string(scope, &search);
        rv.set(v8_str.into());
    }
}

fn url_get_hash(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let hash = url
            .fragment()
            .map(|f| format!("#{}", f))
            .unwrap_or_default();
        let v8_str = to_v8_string(scope, &hash);
        rv.set(v8_str.into());
    }
}

fn url_set_href(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let new_url_str = to_rust_string(scope, args.get(1));
    match Url::parse(&new_url_str) {
        Ok(url) => {
            let v8_str = to_v8_string(scope, url.as_str());
            rv.set(v8_str.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_protocol(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let protocol = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        let protocol = protocol.trim_end_matches(':');
        let _ = url.set_scheme(protocol);
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_username(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let username = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        let _ = url.set_username(&username);
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_password(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let password = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        let _ = url.set_password(Some(&password));
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_host(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let host = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        // Parse host and port if present
        if host.contains(':') {
            let parts: Vec<&str> = host.splitn(2, ':').collect();
            if parts.len() == 2 {
                let _ = url.set_host(Some(parts[0]));
                if let Ok(port) = parts[1].parse::<u16>() {
                    let _ = url.set_port(Some(port));
                }
            }
        } else {
            let _ = url.set_host(Some(&host));
        }
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_hostname(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let hostname = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        let _ = url.set_host(Some(&hostname));
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_port(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let port_str = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        if port_str.is_empty() {
            let _ = url.set_port(None);
        } else if let Ok(port) = port_str.parse::<u16>() {
            let _ = url.set_port(Some(port));
        }
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_pathname(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let pathname = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        url.set_path(&pathname);
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_search(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let search = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        let search = search.trim_start_matches('?');
        if search.is_empty() {
            url.set_query(None);
        } else {
            url.set_query(Some(search));
        }
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_set_hash(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let hash = to_rust_string(scope, args.get(1));

    if let Ok(mut url) = Url::parse(&url_str) {
        let hash = hash.trim_start_matches('#');
        if hash.is_empty() {
            url.set_fragment(None);
        } else {
            url.set_fragment(Some(hash));
        }
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

fn url_to_json(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    if let Ok(url) = Url::parse(&url_str) {
        let v8_str = to_v8_string(scope, url.as_str());
        rv.set(v8_str.into());
    }
}

// URLSearchParams implementation

fn url_search_params_new(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let query = to_rust_string(scope, args.get(0));
    let params = parse_query_string(&query);

    let array = v8::Array::new(scope, params.len() as i32);
    for (i, (key, value)) in params.iter().enumerate() {
        let entry = v8::Array::new(scope, 2);
        let key_str = to_v8_string(scope, key);
        let value_str = to_v8_string(scope, value);
        entry.set_index(scope, 0, key_str.into());
        entry.set_index(scope, 1, value_str.into());
        array.set_index(scope, i as u32, entry.into());
    }
    rv.set(array.into());
}

fn parse_query_string(query: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();
    if query.is_empty() {
        return params;
    }

    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");

        // URL decode
        let key = urlencoding::decode(key).unwrap_or_default().to_string();
        let value = urlencoding::decode(value).unwrap_or_default().to_string();

        params.push((key, value));
    }
    params
}

fn url_search_params_to_string(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let params_array = v8::Local::<v8::Array>::try_from(args.get(0)).unwrap();
    let len = params_array.length();

    let mut parts = Vec::new();
    for i in 0..len {
        let entry = params_array.get_index(scope, i).unwrap();
        let entry_array = v8::Local::<v8::Array>::try_from(entry).unwrap();

        let key = entry_array.get_index(scope, 0).unwrap();
        let value = entry_array.get_index(scope, 1).unwrap();

        let key_str = to_rust_string(scope, key);
        let value_str = to_rust_string(scope, value);

        parts.push(format!(
            "{}={}",
            urlencoding::encode(&key_str),
            urlencoding::encode(&value_str)
        ));
    }

    let result = parts.join("&");
    let v8_str = to_v8_string(scope, &result);
    rv.set(v8_str.into());
}
