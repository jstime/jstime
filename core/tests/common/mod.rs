// Heavily inspired by https://github.com/denoland/rusty_v8/blob/master/tests/test_api.rs
// https://github.com/denoland/deno/blob/master/LICENSE
// MIT License
//
// Copyright (c) 2018-2020 the Deno authors

// We only want to run jstime::init once, but we also
// don't want to run any tests until the runtime is
// rip roaring and ready to execute JS

use lazy_static::lazy_static;

use std::sync::Mutex;

use jstime_core as jstime;

lazy_static! {
    static ref INIT_LOCK: Mutex<u32> = Mutex::new(0);
}

#[must_use]
pub struct SetupGuard {}

impl Drop for SetupGuard {
    fn drop(&mut self) {
        // TODO shutdown process cleanly.
    }
}

pub fn setup() -> SetupGuard {
    let mut g = INIT_LOCK.lock().unwrap();
    *g += 1;
    if *g == 1 {
        jstime::init(None);
    }
    SetupGuard {}
}
