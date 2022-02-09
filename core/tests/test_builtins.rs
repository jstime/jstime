use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_microtask() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof globalThis.queueMicrotask;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }
    #[test]
    fn console() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("Object.keys(console);", "jstime");
        assert_eq!(result.unwrap(), "debug,error,info,log,warn,dir,dirxml,table,trace,group,groupCollapsed,groupEnd,clear,count,countReset,assert,profile,profileEnd,time,timeLog,timeEnd,timeStamp,context");
    }
    #[test]
    fn url() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof URL;", "jstime");
        assert_eq!(result.unwrap(), "function");
        let result = jstime.run_script("typeof URLSearchParams;", "jstime");
        assert_eq!(result.unwrap(), "function");
    }
}
