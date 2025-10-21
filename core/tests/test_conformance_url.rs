// URL API Conformance Tests
// Based on https://url.spec.whatwg.org/

use jstime_core as jstime;

mod common;

#[cfg(test)]
mod conformance_url {
    use super::*;

    #[test]
    fn url_constructor_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof URL;", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn urlsearchparams_constructor_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof URLSearchParams;", "test");
        assert_eq!(result.unwrap(), "function");
    }

    #[test]
    fn url_parses_absolute_url() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com/path'); url.href;",
            "test",
        );
        assert_eq!(result.unwrap(), "https://example.com/path");
    }

    #[test]
    fn url_parses_with_base() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('/path', 'https://example.com'); url.href;",
            "test",
        );
        assert_eq!(result.unwrap(), "https://example.com/path");
    }

    #[test]
    fn url_protocol_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com'); url.protocol;",
            "test",
        );
        assert_eq!(result.unwrap(), "https:");
    }

    #[test]
    fn url_hostname_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com'); url.hostname;",
            "test",
        );
        assert_eq!(result.unwrap(), "example.com");
    }

    #[test]
    fn url_pathname_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com/path'); url.pathname;",
            "test",
        );
        assert_eq!(result.unwrap(), "/path");
    }

    #[test]
    fn url_search_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com?foo=bar'); url.search;",
            "test",
        );
        assert_eq!(result.unwrap(), "?foo=bar");
    }

    #[test]
    fn url_hash_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com#hash'); url.hash;",
            "test",
        );
        assert_eq!(result.unwrap(), "#hash");
    }

    #[test]
    fn url_origin_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com:8080/path'); url.origin;",
            "test",
        );
        assert_eq!(result.unwrap(), "https://example.com:8080");
    }

    #[test]
    fn url_searchparams_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com'); url.searchParams instanceof URLSearchParams;",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn url_searchparams_is_live() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com'); \
             url.searchParams.append('foo', 'bar'); \
             url.search;",
            "test",
        );
        assert_eq!(result.unwrap(), "?foo=bar");
    }

    #[test]
    fn url_setter_updates_href() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com'); \
             url.pathname = '/newpath'; \
             url.href;",
            "test",
        );
        assert_eq!(result.unwrap(), "https://example.com/newpath");
    }

    #[test]
    fn url_tojson_returns_href() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com/path'); \
             JSON.stringify(url);",
            "test",
        );
        assert_eq!(result.unwrap(), "\"https://example.com/path\"");
    }

    #[test]
    fn urlsearchparams_from_string() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&baz=qux'); \
             params.get('foo');",
            "test",
        );
        assert_eq!(result.unwrap(), "bar");
    }

    #[test]
    fn urlsearchparams_append_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams(); \
             params.append('foo', 'bar'); \
             params.get('foo');",
            "test",
        );
        assert_eq!(result.unwrap(), "bar");
    }

    #[test]
    fn urlsearchparams_set_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&foo=baz'); \
             params.set('foo', 'qux'); \
             params.getAll('foo').length;",
            "test",
        );
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn urlsearchparams_delete_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&baz=qux'); \
             params.delete('foo'); \
             params.has('foo');",
            "test",
        );
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn urlsearchparams_has_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar'); \
             params.has('foo');",
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn urlsearchparams_getall_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('foo=bar&foo=baz'); \
             params.getAll('foo').join(',');",
            "test",
        );
        assert_eq!(result.unwrap(), "bar,baz");
    }

    #[test]
    fn urlsearchparams_tostring_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams(); \
             params.append('foo', 'bar'); \
             params.toString();",
            "test",
        );
        assert_eq!(result.unwrap(), "foo=bar");
    }

    #[test]
    fn urlsearchparams_iteration() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const params = new URLSearchParams('a=1&b=2'); \
             let count = 0; \
             for (const [k, v] of params) { count++; } \
             count;",
            "test",
        );
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn url_username_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://user:pass@example.com'); url.username;",
            "test",
        );
        assert_eq!(result.unwrap(), "user");
    }

    #[test]
    fn url_password_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://user:pass@example.com'); url.password;",
            "test",
        );
        assert_eq!(result.unwrap(), "pass");
    }

    #[test]
    fn url_port_property() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com:8080'); url.port;",
            "test",
        );
        assert_eq!(result.unwrap(), "8080");
    }

    #[test]
    fn url_default_port_is_empty() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(
            "const url = new URL('https://example.com'); url.port;",
            "test",
        );
        assert_eq!(result.unwrap(), "");
    }
}
