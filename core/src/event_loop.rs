use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Unique identifier for timers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TimerId(pub(crate) u64);

impl TimerId {
    fn new(id: u64) -> Self {
        TimerId(id)
    }
}

struct Timer {
    callback: v8::Global<v8::Function>,
    fire_at: Instant,
    interval: Option<Duration>,
}

pub(crate) struct EventLoop {
    next_timer_id: u64,
    timers: BTreeMap<TimerId, Timer>,
    timer_queue: BTreeMap<Instant, Vec<TimerId>>,
    timers_to_clear: Rc<RefCell<Vec<TimerId>>>,
}

impl EventLoop {
    pub(crate) fn new(timers_to_clear: Rc<RefCell<Vec<TimerId>>>) -> Self {
        Self {
            next_timer_id: 1,
            timers: BTreeMap::new(),
            timer_queue: BTreeMap::new(),
            timers_to_clear,
        }
    }

    /// Add a timeout that fires once after the specified delay
    pub(crate) fn add_timeout(
        &mut self,
        callback: v8::Global<v8::Function>,
        delay_ms: u64,
    ) -> TimerId {
        let id = TimerId::new(self.next_timer_id);
        self.next_timer_id += 1;

        let fire_at = Instant::now() + Duration::from_millis(delay_ms);
        
        self.timers.insert(
            id,
            Timer {
                callback,
                fire_at,
                interval: None,
            },
        );
        
        self.timer_queue
            .entry(fire_at)
            .or_insert_with(Vec::new)
            .push(id);

        id
    }

    /// Add an interval that fires repeatedly at the specified interval
    pub(crate) fn add_interval(
        &mut self,
        callback: v8::Global<v8::Function>,
        interval_ms: u64,
    ) -> TimerId {
        let id = TimerId::new(self.next_timer_id);
        self.next_timer_id += 1;

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
        
        self.timer_queue
            .entry(fire_at)
            .or_insert_with(Vec::new)
            .push(id);

        id
    }

    /// Actually clear the marked timers
    fn clear_marked_timers(&mut self) {
        let to_clear: Vec<TimerId> = self.timers_to_clear.borrow_mut().drain(..).collect();
        
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

    /// Check if there are any pending timers
    pub(crate) fn has_pending_timers(&self) -> bool {
        !self.timers.is_empty()
    }

    /// Get the next timer fire time
    fn next_fire_time(&self) -> Option<Instant> {
        self.timer_queue.keys().next().copied()
    }

    /// Process timers that are ready to fire
    /// Returns the callbacks that should be executed
    fn collect_ready_timers(&mut self) -> Vec<(TimerId, v8::Global<v8::Function>, bool)> {
        let now = Instant::now();
        let mut ready_callbacks = Vec::new();

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
                        ready_callbacks.push((
                            timer_id,
                            timer.callback.clone(),
                            is_interval,
                        ));
                    }
                }
            }
        }

        ready_callbacks
    }

    /// Reschedule an interval timer
    fn reschedule_interval(&mut self, id: TimerId) {
        if let Some(timer) = self.timers.get_mut(&id) {
            if let Some(interval) = timer.interval {
                let new_fire_at = Instant::now() + interval;
                timer.fire_at = new_fire_at;
                
                self.timer_queue
                    .entry(new_fire_at)
                    .or_insert_with(Vec::new)
                    .push(id);
            }
        }
    }

    /// Run the event loop until there are no more pending operations
    pub(crate) fn run(&mut self, scope: &mut v8::HandleScope) {
        while self.has_pending_timers() {
            // Process all microtasks
            scope.perform_microtask_checkpoint();

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
        }

        // Final microtask checkpoint
        scope.perform_microtask_checkpoint();
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new(Rc::new(RefCell::new(Vec::new())))
    }
}

/// Get the event loop from the isolate state
pub(crate) fn get_event_loop(isolate: &mut v8::Isolate) -> Rc<RefCell<EventLoop>> {
    crate::IsolateState::get(isolate).borrow().event_loop.clone()
}
