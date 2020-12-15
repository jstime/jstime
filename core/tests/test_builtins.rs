// Heavily inspired by https://github.com/denoland/rusty_v8/blob/master/tests/test_api.rs
// https://github.com/denoland/deno/blob/master/LICENSE
// MIT License
//
// Copyright (c) 2018-2020 the Deno authors

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use jstime_core as jstime;

lazy_static! {
    static ref INIT_LOCK: Mutex<u32> = Mutex::new(0);
}

#[must_use]
struct SetupGuard {}

impl Drop for SetupGuard {
    fn drop(&mut self) {
        // TODO shutdown process cleanly.
    }
}

fn setup() -> SetupGuard {
    let mut g = INIT_LOCK.lock().unwrap();
    *g += 1;
    if *g == 1 {
        jstime::init(None);
    }
    SetupGuard {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_microtask() {
        let _setup_guard = setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof globalThis.queueMicrotask;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }
    #[test]
    fn console() {
        let _setup_guard = setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("Object.keys(console);", "jstime");
        assert_eq!(result.unwrap(), "debug,error,info,log,warn,dir,dirxml,table,trace,group,groupCollapsed,groupEnd,clear,count,countReset,assert,profile,profileEnd,time,timeLog,timeEnd,timeStamp,context");
    }
}
