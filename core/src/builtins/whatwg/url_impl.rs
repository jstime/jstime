use url::Url;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
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

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    macro_rules! binding {
        ($name:expr, $fn:ident) => {
            let name = v8::String::new(scope, $name).unwrap();
            let value = v8::Function::new(scope, $fn).unwrap();
            bindings.set(scope, name.into(), value.into());
        };
    }

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
}

// Helper to convert v8 string to Rust string
#[inline]
fn to_rust_string(scope: &mut v8::PinScope, value: v8::Local<v8::Value>) -> String {
    value.to_string(scope).unwrap().to_rust_string_lossy(scope)
}

// Helper to create v8 string from Rust string
#[inline]
fn to_v8_string<'a>(scope: &mut v8::PinScope<'a, '_>, s: &str) -> v8::Local<'a, v8::String> {
    v8::String::new(scope, s).unwrap()
}

// URL parsing function
fn url_parse(
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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
    scope: &mut v8::PinScope,
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

#[inline]
fn parse_query_string(query: &str) -> Vec<(String, String)> {
    if query.is_empty() {
        return Vec::new();
    }

    // Pre-allocate with estimated capacity based on '&' count
    let estimated_capacity = query.matches('&').count() + 1;
    let mut params = Vec::with_capacity(estimated_capacity);

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
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let params_array = v8::Local::<v8::Array>::try_from(args.get(0)).unwrap();
    let len = params_array.length();

    let mut parts = Vec::with_capacity(len as usize);
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
