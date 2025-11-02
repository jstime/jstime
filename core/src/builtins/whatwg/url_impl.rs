use smallvec::SmallVec;
use url::Url;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(url_parse),
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

// Helper to create a URL components object
#[inline]
fn url_to_components_object<'a>(
    scope: &mut v8::PinScope<'a, '_>,
    url: &Url,
) -> v8::Local<'a, v8::Object> {
    let obj = v8::Object::new(scope);

    // Helper macro to set object properties
    macro_rules! set_prop {
        ($name:expr, $value:expr) => {
            let key = v8::String::new(scope, $name).unwrap();
            let val = to_v8_string(scope, $value);
            obj.set(scope, key.into(), val.into());
        };
    }

    set_prop!("href", url.as_str());
    set_prop!("origin", &url.origin().ascii_serialization());
    set_prop!("protocol", &format!("{}:", url.scheme()));
    set_prop!("username", url.username());
    set_prop!("password", url.password().unwrap_or(""));

    let host = url.host_str().unwrap_or("");
    let host_with_port = if let Some(port) = url.port() {
        format!("{}:{}", host, port)
    } else {
        host.to_string()
    };
    set_prop!("host", &host_with_port);
    set_prop!("hostname", host);
    set_prop!(
        "port",
        &url.port().map(|p| p.to_string()).unwrap_or_default()
    );
    set_prop!("pathname", url.path());
    set_prop!(
        "search",
        &url.query().map(|q| format!("?{}", q)).unwrap_or_default()
    );
    set_prop!(
        "hash",
        &url.fragment()
            .map(|f| format!("#{}", f))
            .unwrap_or_default()
    );

    obj
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
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
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
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
            let parts: SmallVec<[&str; 2]> = host.splitn(2, ':').collect();
            if parts.len() == 2 {
                let _ = url.set_host(Some(parts[0]));
                if let Ok(port) = parts[1].parse::<u16>() {
                    let _ = url.set_port(Some(port));
                }
            }
        } else {
            let _ = url.set_host(Some(&host));
        }
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
        let obj = url_to_components_object(scope, &url);
        rv.set(obj.into());
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
