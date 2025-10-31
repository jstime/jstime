use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Unique identifier for timers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TimerId(pub(crate) u64);

/// Type alias for fetch response data: (status, status_text, headers, body_data)
type FetchResponseData = (u16, String, Vec<(String, String)>, Vec<u8>);

struct Timer {
    callback: v8::Global<v8::Function>,
    fire_at: Instant,
    interval: Option<Duration>,
}

/// Pending timer to be added
pub(crate) enum PendingTimer {
    Timeout {
        id: TimerId,
        callback: v8::Global<v8::Function>,
        delay_ms: u64,
    },
    Interval {
        id: TimerId,
        callback: v8::Global<v8::Function>,
        interval_ms: u64,
    },
}

pub(crate) struct EventLoop {
    timers: BTreeMap<TimerId, Timer>,
    timer_queue: BTreeMap<Instant, Vec<TimerId>>,
    timers_to_clear: Rc<RefCell<Vec<TimerId>>>,
    timers_to_add: Rc<RefCell<Vec<PendingTimer>>>,
    pending_fetches: Rc<RefCell<Vec<crate::isolate_state::FetchRequest>>>,
}

impl EventLoop {
    pub(crate) fn new(
        timers_to_clear: Rc<RefCell<Vec<TimerId>>>,
        timers_to_add: Rc<RefCell<Vec<PendingTimer>>>,
        _next_timer_id: Rc<RefCell<u64>>,
        pending_fetches: Rc<RefCell<Vec<crate::isolate_state::FetchRequest>>>,
    ) -> Self {
        Self {
            timers: BTreeMap::new(),
            timer_queue: BTreeMap::new(),
            timers_to_clear,
            timers_to_add,
            pending_fetches,
        }
    }

    /// Process pending timers that were queued for addition
    #[inline]
    fn add_pending_timers(&mut self) {
        let mut pending_borrow = self.timers_to_add.borrow_mut();
        if pending_borrow.is_empty() {
            return;
        }
        let pending: Vec<PendingTimer> = pending_borrow.drain(..).collect();
        drop(pending_borrow);

        for pending_timer in pending {
            match pending_timer {
                PendingTimer::Timeout {
                    id,
                    callback,
                    delay_ms,
                } => {
                    let fire_at = Instant::now() + Duration::from_millis(delay_ms);
                    self.timers.insert(
                        id,
                        Timer {
                            callback,
                            fire_at,
                            interval: None,
                        },
                    );
                    self.timer_queue.entry(fire_at).or_default().push(id);
                }
                PendingTimer::Interval {
                    id,
                    callback,
                    interval_ms,
                } => {
                    let interval = Duration::from_millis(interval_ms);
                    let fire_at = Instant::now() + interval;
                    self.timers.insert(
                        id,
                        Timer {
                            callback,
                            fire_at,
                            interval: Some(interval),
                        },
                    );
                    self.timer_queue.entry(fire_at).or_default().push(id);
                }
            }
        }
    }

    /// Actually clear the marked timers
    #[inline]
    fn clear_marked_timers(&mut self) {
        let mut to_clear_borrow = self.timers_to_clear.borrow_mut();
        if to_clear_borrow.is_empty() {
            return;
        }
        let to_clear: Vec<TimerId> = to_clear_borrow.drain(..).collect();
        drop(to_clear_borrow);

        for id in to_clear {
            if let Some(timer) = self.timers.remove(&id) {
                // Remove from timer queue
                if let Some(timers) = self.timer_queue.get_mut(&timer.fire_at) {
                    timers.retain(|&tid| tid != id);
                    if timers.is_empty() {
                        self.timer_queue.remove(&timer.fire_at);
                    }
                }
            }
        }
    }

    /// Check if there are any pending timers or fetch requests
    pub(crate) fn has_pending_timers(&self) -> bool {
        !self.timers.is_empty() || !self.pending_fetches.borrow().is_empty()
    }

    /// Get the next timer fire time
    fn next_fire_time(&self) -> Option<Instant> {
        self.timer_queue.keys().next().copied()
    }

    /// Process timers that are ready to fire
    /// Returns the callbacks that should be executed
    #[inline]
    fn collect_ready_timers(&mut self) -> Vec<(TimerId, v8::Global<v8::Function>, bool)> {
        let now = Instant::now();
        let mut ready_callbacks = Vec::with_capacity(8);

        // Collect all timers that should fire
        let ready_times: Vec<Instant> = self
            .timer_queue
            .keys()
            .copied()
            .take_while(|&time| time <= now)
            .collect();

        for fire_time in ready_times {
            if let Some(timer_ids) = self.timer_queue.remove(&fire_time) {
                for timer_id in timer_ids {
                    if let Some(timer) = self.timers.get(&timer_id) {
                        let is_interval = timer.interval.is_some();
                        ready_callbacks.push((timer_id, timer.callback.clone(), is_interval));
                    }
                }
            }
        }

        ready_callbacks
    }

    /// Reschedule an interval timer
    #[inline]
    fn reschedule_interval(&mut self, id: TimerId) {
        if let Some(timer) = self.timers.get_mut(&id)
            && let Some(interval) = timer.interval
        {
            let new_fire_at = Instant::now() + interval;
            timer.fire_at = new_fire_at;

            self.timer_queue.entry(new_fire_at).or_default().push(id);
        }
    }

    /// Process pending fetch requests
    #[inline]
    fn process_fetches(&mut self, scope: &mut v8::PinScope) {
        let mut fetches_borrow = self.pending_fetches.borrow_mut();
        if fetches_borrow.is_empty() {
            return;
        }
        let fetches: Vec<crate::isolate_state::FetchRequest> = fetches_borrow.drain(..).collect();
        drop(fetches_borrow);

        // Get the HTTP agent, next_stream_id, and header pool from isolate state
        let isolate: &mut v8::Isolate = scope;
        let state = crate::IsolateState::get(isolate);
        let agent = state.borrow().http_agent.clone();
        let next_stream_id_ref = state.borrow().next_stream_id.clone();
        let header_pool = state.borrow().header_vec_pool.clone();

        for fetch_request in fetches {
            // Execute the HTTP request and get the response (but don't read body yet)
            let result = Self::execute_fetch_streaming(
                &agent,
                &fetch_request.url,
                &fetch_request.method,
                &fetch_request.headers,
                fetch_request.body.as_deref(),
            );

            // Return the request headers to the pool (cleared for reuse)
            let mut headers = fetch_request.headers;
            headers.clear();
            header_pool.put(headers);

            // Resolve the promise with the result
            let resolver = v8::Local::new(scope, &fetch_request.resolver);

            match result {
                Ok((status, status_text, response_headers, body_data)) => {
                    // Allocate a stream ID for this fetch
                    let stream_id = {
                        let mut next_id = next_stream_id_ref.borrow_mut();
                        let id = *next_id;
                        *next_id += 1;
                        id
                    };

                    // Store the body data for streaming
                    let streaming_fetch = crate::isolate_state::StreamingFetch {
                        stream_id,
                        body_data,
                        offset: 0,
                    };

                    {
                        let isolate: &mut v8::Isolate = scope;
                        let state = crate::IsolateState::get(isolate);
                        let streaming_fetches = state.borrow().streaming_fetches.clone();
                        streaming_fetches
                            .borrow_mut()
                            .insert(stream_id, streaming_fetch);
                    }

                    // Create response object
                    let obj = v8::Object::new(scope);

                    // Get or create cached string keys
                    let isolate: &mut v8::Isolate = scope;
                    let state = crate::IsolateState::get(isolate);
                    let cache = state.borrow().string_cache.clone();
                    let mut cache_borrow = cache.borrow_mut();

                    // Set streamId
                    let stream_id_key = v8::String::new(scope, "streamId").unwrap();
                    let stream_id_value = v8::Number::new(scope, stream_id as f64);
                    obj.set(scope, stream_id_key.into(), stream_id_value.into());

                    // Set status (using cached string)
                    let status_key =
                        crate::get_or_create_cached_string!(scope, cache_borrow, status, "status");
                    let status_value = v8::Integer::new(scope, status as i32);
                    obj.set(scope, status_key.into(), status_value.into());

                    // Set statusText (using cached string)
                    let status_text_key = crate::get_or_create_cached_string!(
                        scope,
                        cache_borrow,
                        status_text,
                        "statusText"
                    );
                    let status_text_value = v8::String::new(scope, &status_text).unwrap();
                    obj.set(scope, status_text_key.into(), status_text_value.into());

                    // Set headers (using cached string)
                    let headers_key = crate::get_or_create_cached_string!(
                        scope,
                        cache_borrow,
                        headers,
                        "headers"
                    );

                    drop(cache_borrow);
                    let headers_len = response_headers.len() as i32;
                    let headers_array = v8::Array::new(scope, headers_len);
                    for (i, (key, value)) in response_headers.iter().enumerate() {
                        let entry = v8::Array::new(scope, 2);
                        let key_str = v8::String::new(scope, key).unwrap();
                        let value_str = v8::String::new(scope, value).unwrap();
                        entry.set_index(scope, 0, key_str.into());
                        entry.set_index(scope, 1, value_str.into());
                        headers_array.set_index(scope, i as u32, entry.into());
                    }
                    obj.set(scope, headers_key.into(), headers_array.into());

                    // Return response headers to pool (cleared for reuse)
                    let mut resp_headers = response_headers;
                    resp_headers.clear();
                    header_pool.put(resp_headers);

                    let _ = resolver.resolve(scope, obj.into());
                }
                Err(err) => {
                    let error_msg = v8::String::new(scope, &err).unwrap();
                    let error = v8::Exception::error(scope, error_msg);
                    let _ = resolver.reject(scope, error);
                }
            }
        }
    }

    /// Execute an HTTP request using ureq (streaming version)
    /// Returns (status, status_text, headers, body_data)
    fn execute_fetch_streaming(
        agent: &ureq::Agent,
        url: &str,
        method: &str,
        headers: &[(String, String)],
        body: Option<&str>,
    ) -> Result<FetchResponseData, String> {
        // Build and execute the request based on method
        let response = match method {
            "GET" => {
                let mut req = agent.get(url);
                for (key, value) in headers {
                    req = req.header(key, value);
                }
                req.call()
            }
            "HEAD" => {
                let mut req = agent.head(url);
                for (key, value) in headers {
                    req = req.header(key, value);
                }
                req.call()
            }
            "DELETE" => {
                let mut req = agent.delete(url);
                for (key, value) in headers {
                    req = req.header(key, value);
                }
                req.call()
            }
            "POST" => {
                let mut req = agent.post(url);
                for (key, value) in headers {
                    req = req.header(key, value);
                }
                req.send(body.unwrap_or(""))
            }
            "PUT" => {
                let mut req = agent.put(url);
                for (key, value) in headers {
                    req = req.header(key, value);
                }
                req.send(body.unwrap_or(""))
            }
            "PATCH" => {
                let mut req = agent.patch(url);
                for (key, value) in headers {
                    req = req.header(key, value);
                }
                req.send(body.unwrap_or(""))
            }
            _ => return Err(format!("Unsupported HTTP method: {}", method)),
        };

        match response {
            Ok(mut response) => {
                let status_code = response.status();
                let status = status_code.as_u16();
                let status_text = status_code
                    .canonical_reason()
                    .unwrap_or("Unknown")
                    .to_string();

                // Get headers - ureq 3.x uses http crate's HeaderMap
                let headers_map = response.headers();
                let mut response_headers = Vec::with_capacity(headers_map.len());
                for (name, value) in headers_map {
                    if let Ok(value_str) = value.to_str() {
                        response_headers.push((name.as_str().to_string(), value_str.to_string()));
                    }
                }

                // Read the body into a vector
                match response.body_mut().read_to_vec() {
                    Ok(body_data) => Ok((status, status_text, response_headers, body_data)),
                    Err(e) => Err(format!("Failed to read response body: {}", e)),
                }
            }
            Err(err) => Err(format!("Network error: {}", err)),
        }
    }

    /// Run the event loop until there are no more pending operations
    pub(crate) fn run(&mut self, scope: &mut v8::PinScope) {
        // First, add any pending timers
        self.add_pending_timers();

        while self.has_pending_timers() {
            // Process pending fetch requests
            self.process_fetches(scope);

            // Process all microtasks
            scope.perform_microtask_checkpoint();

            // Check again if we have pending operations after processing fetches
            if !self.has_pending_timers() {
                break;
            }

            // Get next fire time
            if let Some(next_time) = self.next_fire_time() {
                let now = Instant::now();
                if next_time > now {
                    // Sleep until next timer
                    std::thread::sleep(next_time - now);
                }
            }

            // Collect and execute ready timers
            let ready_timers = self.collect_ready_timers();

            for (timer_id, callback, is_interval) in ready_timers {
                let callback_local = v8::Local::new(scope, &callback);
                let recv = v8::undefined(scope).into();
                let _ = callback_local.call(scope, recv, &[]);

                if is_interval {
                    // Reschedule interval timers
                    self.reschedule_interval(timer_id);
                } else {
                    // Remove one-shot timers
                    self.timers.remove(&timer_id);
                }
            }

            // Clear any timers that were marked for clearing during callbacks
            self.clear_marked_timers();

            // Add any timers that were queued during callbacks
            self.add_pending_timers();
        }

        // Final microtask checkpoint
        scope.perform_microtask_checkpoint();
    }

    /// Process ready timers without blocking (suitable for REPL)
    /// This method executes timers that are ready to fire and returns immediately
    pub(crate) fn tick(&mut self, scope: &mut v8::PinScope) {
        // Add any pending timers
        self.add_pending_timers();

        // Process pending fetch requests
        self.process_fetches(scope);

        // Process all microtasks
        scope.perform_microtask_checkpoint();

        // Collect and execute ready timers (without sleeping)
        let ready_timers = self.collect_ready_timers();

        for (timer_id, callback, is_interval) in ready_timers {
            let callback_local = v8::Local::new(scope, &callback);
            let recv = v8::undefined(scope).into();
            let _ = callback_local.call(scope, recv, &[]);

            if is_interval {
                // Reschedule interval timers
                self.reschedule_interval(timer_id);
            } else {
                // Remove one-shot timers
                self.timers.remove(&timer_id);
            }
        }

        // Clear any timers that were marked for clearing during callbacks
        self.clear_marked_timers();

        // Add any timers that were queued during callbacks
        self.add_pending_timers();

        // Final microtask checkpoint
        scope.perform_microtask_checkpoint();
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new(
            Rc::new(RefCell::new(Vec::new())),
            Rc::new(RefCell::new(Vec::new())),
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(Vec::new())),
        )
    }
}

/// Get the event loop from the isolate state
pub(crate) fn get_event_loop(scope: &mut v8::PinScope) -> Rc<RefCell<EventLoop>> {
    let isolate: &mut v8::Isolate = scope;
    crate::IsolateState::get(isolate)
        .borrow()
        .event_loop
        .clone()
}
