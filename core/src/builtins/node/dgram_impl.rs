use std::net::UdpSocket;

pub(crate) fn get_external_references() -> Vec<v8::ExternalReference> {
    vec![
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_create_socket),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_bind),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_send),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_close),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_address),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_set_broadcast),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_set_ttl),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_set_multicast_ttl),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_set_multicast_loopback),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_add_membership),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_drop_membership),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_recv),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_register_for_messages),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_unregister_for_messages),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_ref),
        },
        v8::ExternalReference {
            function: v8::MapFnTo::map_fn_to(dgram_unref),
        },
    ]
}

pub(crate) fn register_bindings(scope: &mut v8::PinScope, bindings: v8::Local<v8::Object>) {
    let name = v8::String::new(scope, "dgramCreateSocket").unwrap();
    let value = v8::Function::new(scope, dgram_create_socket).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramBind").unwrap();
    let value = v8::Function::new(scope, dgram_bind).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramSend").unwrap();
    let value = v8::Function::new(scope, dgram_send).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramClose").unwrap();
    let value = v8::Function::new(scope, dgram_close).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramAddress").unwrap();
    let value = v8::Function::new(scope, dgram_address).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramSetBroadcast").unwrap();
    let value = v8::Function::new(scope, dgram_set_broadcast).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramSetTTL").unwrap();
    let value = v8::Function::new(scope, dgram_set_ttl).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramSetMulticastTTL").unwrap();
    let value = v8::Function::new(scope, dgram_set_multicast_ttl).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramSetMulticastLoopback").unwrap();
    let value = v8::Function::new(scope, dgram_set_multicast_loopback).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramAddMembership").unwrap();
    let value = v8::Function::new(scope, dgram_add_membership).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramDropMembership").unwrap();
    let value = v8::Function::new(scope, dgram_drop_membership).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramRecv").unwrap();
    let value = v8::Function::new(scope, dgram_recv).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramRegisterForMessages").unwrap();
    let value = v8::Function::new(scope, dgram_register_for_messages).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramUnregisterForMessages").unwrap();
    let value = v8::Function::new(scope, dgram_unregister_for_messages).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramRef").unwrap();
    let value = v8::Function::new(scope, dgram_ref).unwrap();
    bindings.set(scope, name.into(), value.into());

    let name = v8::String::new(scope, "dgramUnref").unwrap();
    let value = v8::Function::new(scope, dgram_unref).unwrap();
    bindings.set(scope, name.into(), value.into());
}

/// Create a UDP socket
/// Returns a socket ID (stored in isolate state)
#[inline]
fn dgram_create_socket(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "dgramCreateSocket") {
        return;
    }

    // Get socket type: "udp4" or "udp6"
    let type_arg = args.get(0);
    let Some(socket_type) = crate::error::to_rust_string_or_throw(scope, type_arg, "type") else {
        return;
    };

    // Validate socket type
    if socket_type != "udp4" && socket_type != "udp6" {
        crate::error::throw_error(
            scope,
            &format!(
                "Invalid socket type: {}. Must be 'udp4' or 'udp6'",
                socket_type
            ),
        );
        return;
    }

    // Create an unbound socket
    // For now, we just return the socket type as we'll bind later
    // We store socket configuration in the JS object
    let result = v8::String::new(scope, &socket_type).unwrap();
    retval.set(result.into());
}

/// Bind a UDP socket to a port and address
/// Args: socketType, port, address
/// Returns the bound socket file descriptor as an external
#[inline]
fn dgram_bind(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 3, "dgramBind") {
        return;
    }

    // Get socket type
    let type_arg = args.get(0);
    let Some(socket_type) = crate::error::to_rust_string_or_throw(scope, type_arg, "type") else {
        return;
    };

    // Get port
    let port_arg = args.get(1);
    let port = if port_arg.is_number() {
        port_arg.number_value(scope).unwrap() as u16
    } else {
        0 // Default to OS-assigned port
    };

    // Get address
    let addr_arg = args.get(2);
    let address = if addr_arg.is_null_or_undefined() {
        if socket_type == "udp6" {
            "::".to_string()
        } else {
            "0.0.0.0".to_string()
        }
    } else {
        let Some(addr) = crate::error::to_rust_string_or_throw(scope, addr_arg, "address") else {
            return;
        };
        addr
    };

    // Create and bind the socket
    let bind_addr = format!("{}:{}", address, port);
    match UdpSocket::bind(&bind_addr) {
        Ok(socket) => {
            // Set non-blocking mode for async operations
            if let Err(e) = socket.set_nonblocking(true) {
                crate::error::throw_error(scope, &format!("Failed to set non-blocking: {}", e));
                return;
            }

            // Store the socket in a Box and create an external reference
            let socket_box = Box::new(socket);
            let socket_ptr = Box::into_raw(socket_box);

            // Create an external value to store the socket pointer
            let external = v8::External::new(scope, socket_ptr as *mut std::ffi::c_void);
            retval.set(external.into());
        }
        Err(e) => {
            crate::error::throw_error(scope, &format!("Failed to bind socket: {}", e));
        }
    }
}

/// Send data through a UDP socket
/// Args: socketExternal, buffer, offset, length, port, address
#[inline]
fn dgram_send(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 6, "dgramSend") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    // Get the buffer
    let buffer_arg = args.get(1);
    let data = if buffer_arg.is_uint8_array() {
        let Some(uint8_array) = v8::Local::<v8::Uint8Array>::try_from(buffer_arg).ok() else {
            crate::error::throw_type_error(scope, "Failed to convert to Uint8Array");
            return;
        };
        let byte_length = uint8_array.byte_length();
        let mut buffer = vec![0u8; byte_length];
        let copied = uint8_array.copy_contents(&mut buffer);
        if copied != buffer.len() {
            crate::error::throw_error(scope, "Failed to copy buffer data");
            return;
        }
        buffer
    } else if buffer_arg.is_string() {
        let Some(data_str) = crate::error::to_rust_string_or_throw(scope, buffer_arg, "buffer")
        else {
            return;
        };
        data_str.into_bytes()
    } else {
        crate::error::throw_type_error(scope, "Buffer must be a Uint8Array or string");
        return;
    };

    // Get offset and length
    let offset_arg = args.get(2);
    let offset = if offset_arg.is_number() {
        offset_arg.number_value(scope).unwrap() as usize
    } else {
        0
    };

    let length_arg = args.get(3);
    let length = if length_arg.is_number() {
        length_arg.number_value(scope).unwrap() as usize
    } else {
        data.len()
    };

    // Validate offset and length
    if offset > data.len() || offset + length > data.len() {
        crate::error::throw_range_error(scope, "Offset and length exceed buffer bounds");
        return;
    }

    // Get port and address
    let port_arg = args.get(4);
    let port = if port_arg.is_number() {
        port_arg.number_value(scope).unwrap() as u16
    } else {
        crate::error::throw_type_error(scope, "Port must be a number");
        return;
    };

    let addr_arg = args.get(5);
    let Some(address) = crate::error::to_rust_string_or_throw(scope, addr_arg, "address") else {
        return;
    };

    // Send the data
    let target_addr = format!("{}:{}", address, port);
    let socket = unsafe { &*socket_ptr };

    match socket.send_to(&data[offset..offset + length], &target_addr) {
        Ok(bytes_sent) => {
            let result = v8::Number::new(scope, bytes_sent as f64);
            retval.set(result.into());
        }
        Err(e) => {
            crate::error::throw_error(scope, &format!("Failed to send: {}", e));
        }
    }
}

/// Close a UDP socket
/// Args: socketExternal
#[inline]
fn dgram_close(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "dgramClose") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        // Already closed
        return;
    }

    // Drop the socket to close it
    unsafe {
        drop(Box::from_raw(socket_ptr));
    }
}

/// Get the address info of a bound socket
/// Args: socketExternal
/// Returns: { address, family, port }
#[inline]
fn dgram_address(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "dgramAddress") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let socket = unsafe { &*socket_ptr };

    match socket.local_addr() {
        Ok(addr) => {
            let obj = v8::Object::new(scope);

            let address_key = v8::String::new(scope, "address").unwrap();
            let address_value = v8::String::new(scope, &addr.ip().to_string()).unwrap();
            obj.set(scope, address_key.into(), address_value.into());

            let family_key = v8::String::new(scope, "family").unwrap();
            let family_value = if addr.is_ipv4() {
                v8::String::new(scope, "IPv4").unwrap()
            } else {
                v8::String::new(scope, "IPv6").unwrap()
            };
            obj.set(scope, family_key.into(), family_value.into());

            let port_key = v8::String::new(scope, "port").unwrap();
            let port_value = v8::Number::new(scope, addr.port() as f64);
            obj.set(scope, port_key.into(), port_value.into());

            retval.set(obj.into());
        }
        Err(e) => {
            crate::error::throw_error(scope, &format!("Failed to get address: {}", e));
        }
    }
}

/// Set broadcast option on socket
/// Args: socketExternal, flag (boolean)
#[inline]
fn dgram_set_broadcast(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "dgramSetBroadcast") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let flag_arg = args.get(1);
    let flag = flag_arg.is_true();

    let socket = unsafe { &*socket_ptr };

    if let Err(e) = socket.set_broadcast(flag) {
        crate::error::throw_error(scope, &format!("Failed to set broadcast: {}", e));
    }
}

/// Set TTL (Time To Live) on socket
/// Args: socketExternal, ttl (number)
#[inline]
fn dgram_set_ttl(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "dgramSetTTL") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let ttl_arg = args.get(1);
    let ttl = if ttl_arg.is_number() {
        ttl_arg.number_value(scope).unwrap() as u32
    } else {
        crate::error::throw_type_error(scope, "TTL must be a number");
        return;
    };

    let socket = unsafe { &*socket_ptr };

    if let Err(e) = socket.set_ttl(ttl) {
        crate::error::throw_error(scope, &format!("Failed to set TTL: {}", e));
    }
}

/// Set multicast TTL on socket
/// Args: socketExternal, ttl (number)
#[inline]
fn dgram_set_multicast_ttl(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "dgramSetMulticastTTL") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let ttl_arg = args.get(1);
    let ttl = if ttl_arg.is_number() {
        ttl_arg.number_value(scope).unwrap() as u32
    } else {
        crate::error::throw_type_error(scope, "TTL must be a number");
        return;
    };

    let socket = unsafe { &*socket_ptr };

    if let Err(e) = socket.set_multicast_ttl_v4(ttl) {
        crate::error::throw_error(scope, &format!("Failed to set multicast TTL: {}", e));
    }
}

/// Set multicast loopback on socket
/// Args: socketExternal, flag (boolean)
#[inline]
fn dgram_set_multicast_loopback(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "dgramSetMulticastLoopback") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let flag_arg = args.get(1);
    let flag = flag_arg.is_true();

    let socket = unsafe { &*socket_ptr };

    if let Err(e) = socket.set_multicast_loop_v4(flag) {
        crate::error::throw_error(scope, &format!("Failed to set multicast loopback: {}", e));
    }
}

/// Add membership to a multicast group
/// Args: socketExternal, multicastAddress, interfaceAddress
#[inline]
fn dgram_add_membership(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 3, "dgramAddMembership") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let multicast_arg = args.get(1);
    let Some(multicast_addr) =
        crate::error::to_rust_string_or_throw(scope, multicast_arg, "multicastAddress")
    else {
        return;
    };

    let interface_arg = args.get(2);
    let interface_addr = if interface_arg.is_null_or_undefined() {
        "0.0.0.0".to_string()
    } else {
        let Some(addr) =
            crate::error::to_rust_string_or_throw(scope, interface_arg, "interfaceAddress")
        else {
            return;
        };
        addr
    };

    let socket = unsafe { &*socket_ptr };

    // Parse addresses
    let multicast_ip: std::net::Ipv4Addr = match multicast_addr.parse() {
        Ok(ip) => ip,
        Err(e) => {
            crate::error::throw_error(scope, &format!("Invalid multicast address: {}", e));
            return;
        }
    };

    let interface_ip: std::net::Ipv4Addr = match interface_addr.parse() {
        Ok(ip) => ip,
        Err(e) => {
            crate::error::throw_error(scope, &format!("Invalid interface address: {}", e));
            return;
        }
    };

    if let Err(e) = socket.join_multicast_v4(&multicast_ip, &interface_ip) {
        crate::error::throw_error(scope, &format!("Failed to join multicast group: {}", e));
    }
}

/// Drop membership from a multicast group
/// Args: socketExternal, multicastAddress, interfaceAddress
#[inline]
fn dgram_drop_membership(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 3, "dgramDropMembership") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let multicast_arg = args.get(1);
    let Some(multicast_addr) =
        crate::error::to_rust_string_or_throw(scope, multicast_arg, "multicastAddress")
    else {
        return;
    };

    let interface_arg = args.get(2);
    let interface_addr = if interface_arg.is_null_or_undefined() {
        "0.0.0.0".to_string()
    } else {
        let Some(addr) =
            crate::error::to_rust_string_or_throw(scope, interface_arg, "interfaceAddress")
        else {
            return;
        };
        addr
    };

    let socket = unsafe { &*socket_ptr };

    // Parse addresses
    let multicast_ip: std::net::Ipv4Addr = match multicast_addr.parse() {
        Ok(ip) => ip,
        Err(e) => {
            crate::error::throw_error(scope, &format!("Invalid multicast address: {}", e));
            return;
        }
    };

    let interface_ip: std::net::Ipv4Addr = match interface_addr.parse() {
        Ok(ip) => ip,
        Err(e) => {
            crate::error::throw_error(scope, &format!("Invalid interface address: {}", e));
            return;
        }
    };

    if let Err(e) = socket.leave_multicast_v4(&multicast_ip, &interface_ip) {
        crate::error::throw_error(scope, &format!("Failed to leave multicast group: {}", e));
    }
}

/// Receive data from a UDP socket (non-blocking)
/// Args: socketExternal
/// Returns: { data: Uint8Array, rinfo: { address, family, port, size } } or null if no data
#[inline]
fn dgram_recv(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "dgramRecv") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    let socket = unsafe { &*socket_ptr };

    // Use a buffer for receiving data (64KB max UDP packet size)
    let mut buf = vec![0u8; 65536];

    match socket.recv_from(&mut buf) {
        Ok((size, addr)) => {
            // Create the result object
            let obj = v8::Object::new(scope);

            // Create Uint8Array with the received data
            let data_slice = buf[..size].to_vec();
            let backing_store =
                v8::ArrayBuffer::new_backing_store_from_vec(data_slice).make_shared();
            let array_buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
            let uint8_array = v8::Uint8Array::new(scope, array_buffer, 0, size).unwrap();

            let data_key = v8::String::new(scope, "data").unwrap();
            obj.set(scope, data_key.into(), uint8_array.into());

            // Create rinfo object
            let rinfo = v8::Object::new(scope);

            let address_key = v8::String::new(scope, "address").unwrap();
            let address_value = v8::String::new(scope, &addr.ip().to_string()).unwrap();
            rinfo.set(scope, address_key.into(), address_value.into());

            let family_key = v8::String::new(scope, "family").unwrap();
            let family_value = if addr.is_ipv4() {
                v8::String::new(scope, "IPv4").unwrap()
            } else {
                v8::String::new(scope, "IPv6").unwrap()
            };
            rinfo.set(scope, family_key.into(), family_value.into());

            let port_key = v8::String::new(scope, "port").unwrap();
            let port_value = v8::Number::new(scope, addr.port() as f64);
            rinfo.set(scope, port_key.into(), port_value.into());

            let size_key = v8::String::new(scope, "size").unwrap();
            let size_value = v8::Number::new(scope, size as f64);
            rinfo.set(scope, size_key.into(), size_value.into());

            let rinfo_key = v8::String::new(scope, "rinfo").unwrap();
            obj.set(scope, rinfo_key.into(), rinfo.into());

            retval.set(obj.into());
        }
        Err(e) => {
            // For non-blocking sockets, WouldBlock means no data available
            if e.kind() == std::io::ErrorKind::WouldBlock {
                retval.set(v8::null(scope).into());
            } else {
                crate::error::throw_error(scope, &format!("Failed to receive: {}", e));
            }
        }
    }
}

/// Register a socket to receive message callbacks from the event loop
/// Args: socketExternal, callback (function that receives data and rinfo)
/// Returns: socket_id (number) to use for unregistering
#[inline]
fn dgram_register_for_messages(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 2, "dgramRegisterForMessages") {
        return;
    }

    // Get the socket external
    let socket_arg = args.get(0);
    let Some(socket_external) = v8::Local::<v8::External>::try_from(socket_arg).ok() else {
        crate::error::throw_type_error(scope, "Invalid socket reference");
        return;
    };

    let socket_ptr = socket_external.value() as *mut UdpSocket;
    if socket_ptr.is_null() {
        crate::error::throw_error(scope, "Socket has been closed");
        return;
    }

    // Get the callback function
    let callback_arg = args.get(1);
    let Some(callback) = v8::Local::<v8::Function>::try_from(callback_arg).ok() else {
        crate::error::throw_type_error(scope, "Callback must be a function");
        return;
    };

    // Get isolate state and register the socket
    let isolate: &mut v8::Isolate = scope;
    let state = crate::IsolateState::get(isolate);

    let socket_id = {
        let state_ref = state.borrow();
        let next_dgram_socket_id_ref = state_ref.next_dgram_socket_id.clone();
        drop(state_ref);
        let mut next_id = next_dgram_socket_id_ref.borrow_mut();
        let id = *next_id;
        *next_id += 1;
        id
    };

    let callback_global = v8::Global::new(scope, callback);

    let active_socket = crate::isolate_state::ActiveDgramSocket {
        socket_id,
        socket_ptr,
        callback: callback_global,
        is_ref: true, // By default, sockets keep the event loop alive
    };

    state
        .borrow()
        .active_dgram_sockets
        .borrow_mut()
        .insert(socket_id, active_socket);

    let result = v8::Number::new(scope, socket_id as f64);
    retval.set(result.into());
}

/// Unregister a socket from receiving message callbacks
/// Args: socket_id (number)
#[inline]
fn dgram_unregister_for_messages(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "dgramUnregisterForMessages") {
        return;
    }

    let socket_id_arg = args.get(0);
    let socket_id = if socket_id_arg.is_number() {
        socket_id_arg.number_value(scope).unwrap() as u64
    } else {
        crate::error::throw_type_error(scope, "Socket ID must be a number");
        return;
    };

    // Get isolate state and unregister the socket
    let isolate: &mut v8::Isolate = scope;
    let state = crate::IsolateState::get(isolate);
    state
        .borrow()
        .active_dgram_sockets
        .borrow_mut()
        .remove(&socket_id);
}

/// Reference the socket to keep the event loop alive
/// Args: socket_id (number)
#[inline]
fn dgram_ref(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "dgramRef") {
        return;
    }

    let socket_id_arg = args.get(0);
    let socket_id = if socket_id_arg.is_number() {
        socket_id_arg.number_value(scope).unwrap() as u64
    } else {
        crate::error::throw_type_error(scope, "Socket ID must be a number");
        return;
    };

    // Get isolate state and set is_ref to true
    let isolate: &mut v8::Isolate = scope;
    let state = crate::IsolateState::get(isolate);
    if let Some(socket) = state
        .borrow()
        .active_dgram_sockets
        .borrow_mut()
        .get_mut(&socket_id)
    {
        socket.is_ref = true;
    }
}

/// Unreference the socket to allow the event loop to exit
/// Args: socket_id (number)
#[inline]
fn dgram_unref(
    scope: &mut v8::PinScope,
    args: v8::FunctionCallbackArguments,
    _retval: v8::ReturnValue,
) {
    if !crate::error::check_arg_count(scope, &args, 1, "dgramUnref") {
        return;
    }

    let socket_id_arg = args.get(0);
    let socket_id = if socket_id_arg.is_number() {
        socket_id_arg.number_value(scope).unwrap() as u64
    } else {
        crate::error::throw_type_error(scope, "Socket ID must be a number");
        return;
    };

    // Get isolate state and set is_ref to false
    let isolate: &mut v8::Isolate = scope;
    let state = crate::IsolateState::get(isolate);
    if let Some(socket) = state
        .borrow()
        .active_dgram_sockets
        .borrow_mut()
        .get_mut(&socket_id)
    {
        socket.is_ref = false;
    }
}
