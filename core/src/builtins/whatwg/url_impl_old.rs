use ada_url::Url;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::cell::RefCell;

thread_local! {
    // Cache of parsed URLs to avoid reparsing the same URL string
    // Using FxHashMap for faster hashing of string keys
    static URL_CACHE: RefCell<FxHashMap<String, Url>> = RefCell::new(FxHashMap::default());
}

// Helper to evict half the cache when size limit is reached
#[inline]
fn evict_cache_if_full(cache_map: &mut FxHashMap<String, Url>) {
    const MAX_CACHE_SIZE: usize = 512;
    if cache_map.len() >= MAX_CACHE_SIZE {
        // Clear half the cache to maintain some entries
        // Note: HashMap iteration order is non-deterministic, but this is acceptable
        // for a simple cache eviction strategy
        let keys_to_remove: Vec<String> =
            cache_map.keys().take(MAX_CACHE_SIZE / 2).cloned().collect();
        for key in keys_to_remove {
            cache_map.remove(&key);
        }
    }
}

// Helper to get or parse a URL with caching
#[inline]
fn get_or_parse_url(url_str: &str) -> Result<Url, ()> {
    URL_CACHE.with(|cache| {
        let mut cache_map = cache.borrow_mut();

        // Check if URL is in cache
        if let Some(cached_url) = cache_map.get(url_str) {
            return Ok(cached_url.clone());
        }

        // Parse URL (ada-url requires base parameter, use None for absolute URLs)
        match Url::parse(url_str, None) {
            Ok(url) => {
                evict_cache_if_full(&mut cache_map);
                cache_map.insert(url_str.to_string(), url.clone());
                Ok(url)
            }
            Err(_) => Err(()),
        }
    })
}

// Helper to update cache with modified URL
#[inline]
fn update_url_cache(url: &Url) {
    let url_str = url.as_str().to_string();
    URL_CACHE.with(|cache| {
        let mut cache_map = cache.borrow_mut();
        evict_cache_if_full(&mut cache_map);
        cache_map.insert(url_str, url.clone());
    });
}

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

    // ada-url provides all components directly
    set_prop!("href", url.as_str());
    set_prop!("origin", &url.origin());
    set_prop!("protocol", &url.protocol());
    set_prop!("username", &url.username());
    set_prop!("password", &url.password());
    set_prop!("host", &url.host());
    set_prop!("hostname", &url.hostname());
    set_prop!("port", &url.port());
    set_prop!("pathname", &url.pathname());
    set_prop!("search", &url.search());
    set_prop!("hash", &url.hash());

    obj
}

// URL parsing function
fn url_parse(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));

    // ada-url uses an Option<&str> for the base parameter
    let parsed = if args.length() > 1 && !args.get(1).is_undefined() {
        let base_str = to_rust_string(scope, args.get(1));
        Url::parse(&url_str, Some(&base_str))
    } else {
        Url::parse(&url_str, None)
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
    match Url::parse(&new_url_str, None) {
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

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            let _ = url.set_protocol(&protocol);
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_username(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let username = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            let _ = url.set_username(Some(&username));
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_password(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let password = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            let _ = url.set_password(Some(&password));
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_host(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let host = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            let _ = url.set_host(Some(&host));
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_hostname(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let hostname = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            let _ = url.set_hostname(Some(&hostname));
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_port(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let port_str = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            if port_str.is_empty() {
                let _ = url.set_port(None);
            } else {
                let _ = url.set_port(Some(&port_str));
            }
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_pathname(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let pathname = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            let _ = url.set_pathname(Some(&pathname));
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_search(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let search = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            if search.is_empty() || search == "?" {
                url.set_search(None);
            } else {
                url.set_search(Some(&search));
            }
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
    }
}

fn url_set_hash(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url_str = to_rust_string(scope, args.get(0));
    let hash = to_rust_string(scope, args.get(1));

    match get_or_parse_url(&url_str) {
        Ok(mut url) => {
            if hash.is_empty() || hash == "#" {
                url.set_hash(None);
            } else {
                url.set_hash(Some(&hash));
            }
            update_url_cache(&url);
            let obj = url_to_components_object(scope, &url);
            rv.set(obj.into());
        }
        Err(_) => {
            rv.set(v8::null(scope).into());
        }
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

        // Split on first '=' only
        if let Some(eq_pos) = pair.find('=') {
            let key = &pair[..eq_pos];
            let value = &pair[eq_pos + 1..];

            // URL decode - use Cow to avoid allocation when no decoding needed
            let key_decoded = urlencoding::decode(key).unwrap_or(Cow::Borrowed(key));
            let value_decoded = urlencoding::decode(value).unwrap_or(Cow::Borrowed(value));

            params.push((key_decoded.into_owned(), value_decoded.into_owned()));
        } else {
            // No '=' found, treat whole pair as key with empty value
            let key_decoded = urlencoding::decode(pair).unwrap_or(Cow::Borrowed(pair));
            params.push((key_decoded.into_owned(), String::new()));
        }
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

    if len == 0 {
        let v8_str = to_v8_string(scope, "");
        rv.set(v8_str.into());
        return;
    }

    // Pre-allocate with estimated size to reduce reallocations
    // Estimate: 20 bytes per parameter is reasonable for typical query strings
    // (e.g., "key=value" is 9 bytes, encoded values are usually longer)
    let mut result = String::with_capacity(len as usize * 20);

    for i in 0..len {
        if i > 0 {
            result.push('&');
        }

        let entry = params_array.get_index(scope, i).unwrap();
        let entry_array = v8::Local::<v8::Array>::try_from(entry).unwrap();

        let key = entry_array.get_index(scope, 0).unwrap();
        let value = entry_array.get_index(scope, 1).unwrap();

        let key_str = to_rust_string(scope, key);
        let value_str = to_rust_string(scope, value);

        result.push_str(&urlencoding::encode(&key_str));
        result.push('=');
        result.push_str(&urlencoding::encode(&value_str));
    }

    let v8_str = to_v8_string(scope, &result);
    rv.set(v8_str.into());
}
