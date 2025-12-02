use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dgram_module_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const mod = globalThis.__node_modules['node:dgram'];
            typeof mod === 'object' && typeof mod.createSocket === 'function'
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_create_socket_udp4() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket instanceof dgram.Socket
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_create_socket_udp6() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp6');
            socket instanceof dgram.Socket
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_create_socket_with_options() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket({ type: 'udp4', recvBufferSize: 1024 });
            socket instanceof dgram.Socket
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_bind() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            let bound = false;
            socket.on('listening', () => { bound = true; });
            socket.bind(0);
            // Wait for async listening event
            socket.close();
            bound
            "#,
            "test",
        );
        // The listening event is async, so it might not have fired yet
        // This test just ensures bind doesn't throw
        assert!(result.is_ok());
    }

    #[test]
    fn test_dgram_socket_address() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            const addr = socket.address();
            const result = typeof addr.address === 'string' && typeof addr.port === 'number' && addr.family === 'IPv4';
            socket.close();
            result
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_close() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            socket.close();
            'closed'
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "closed");
    }

    #[test]
    fn test_dgram_socket_send_basic() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            const addr = socket.address();
            // Send a message to ourselves
            let sent = false;
            socket.send('hello', addr.port, '127.0.0.1', (err, bytes) => {
                sent = !err && bytes === 5;
            });
            socket.close();
            sent
            "#,
            "test",
        );
        // The callback is async, so sent might still be false
        assert!(result.is_ok());
    }

    #[test]
    fn test_dgram_socket_send_buffer() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            const addr = socket.address();
            const buffer = new Uint8Array([72, 101, 108, 108, 111]);
            socket.send(buffer, 0, buffer.length, addr.port, '127.0.0.1');
            socket.close();
            'sent'
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "sent");
    }

    #[test]
    fn test_dgram_socket_set_broadcast() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            socket.setBroadcast(true);
            socket.setBroadcast(false);
            socket.close();
            'success'
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_dgram_socket_set_ttl() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            socket.setTTL(64);
            socket.close();
            'success'
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_dgram_socket_buffer_size() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket({ type: 'udp4', recvBufferSize: 2048 });
            const recvSize = socket.getRecvBufferSize();
            socket.setRecvBufferSize(4096);
            const newRecvSize = socket.getRecvBufferSize();
            recvSize === 2048 && newRecvSize === 4096
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_ref_unref() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            const refResult = socket.ref();
            const unrefResult = socket.unref();
            // ref() and unref() should return the socket for chaining
            refResult === socket && unrefResult === socket
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_invalid_type() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            let error = null;
            try {
                const socket = dgram.createSocket('invalid');
            } catch (e) {
                error = e;
            }
            error !== null && error.message.includes('Bad socket type')
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_address_not_bound() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            let error = null;
            try {
                socket.address();
            } catch (e) {
                error = e;
            }
            error !== null && error.message.includes('not bound')
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_on_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            // Test that on() returns the socket for chaining
            const result = socket.on('message', () => {});
            result === socket
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_once_method() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            // Test that once() returns the socket for chaining
            const result = socket.once('listening', () => {});
            result === socket
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_message_callback_registration() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        // Test that registering a message listener works without error
        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const server = dgram.createSocket('udp4');
            let callbackRegistered = false;
            
            server.on('message', (msg, rinfo) => {
                // This callback won't be called in this test
                callbackRegistered = true;
            });
            
            server.bind(0);
            const addr = server.address();
            
            // Verify the socket is bound and the callback was registered
            // (the callback won't fire because we close immediately)
            server.close();
            
            addr.port > 0 && addr.family === 'IPv4'
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_dgram_socket_unref_allows_exit() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            
            // Unref should allow the process to exit even with bound socket
            socket.unref();
            
            'unref_called'
            "#,
            "test",
        );
        // This test should complete without hanging since unref() was called
        assert_eq!(result.unwrap(), "unref_called");
    }

    #[test]
    fn test_dgram_socket_ref_after_unref() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);

        let result = jstime.run_script(
            r#"
            const dgram = globalThis.__node_modules['node:dgram'];
            const socket = dgram.createSocket('udp4');
            socket.bind(0);
            
            // Unref, then ref, then unref again - should exit
            socket.unref();
            socket.ref();
            socket.unref();
            socket.close();
            
            'ref_unref_chain'
            "#,
            "test",
        );
        assert_eq!(result.unwrap(), "ref_unref_chain");
    }
}
