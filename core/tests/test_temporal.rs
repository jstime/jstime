use jstime_core as jstime;

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporal_namespace_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Temporal;", "jstime");
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn temporal_now_exists() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Temporal.Now;", "jstime");
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn temporal_now_instant() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Temporal.Now.instant;", "jstime");
        assert_eq!(result.unwrap(), "function");
        let result = jstime.run_script(
            "const instant = Temporal.Now.instant(); typeof instant;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "object");
    }

    #[test]
    fn temporal_plain_date() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Temporal.PlainDate;", "jstime");
        assert_eq!(result.unwrap(), "function");
        let result = jstime.run_script(
            "const date = new Temporal.PlainDate(2025, 10, 13); date.year;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "2025");
        let result = jstime.run_script("date.month;", "jstime");
        assert_eq!(result.unwrap(), "10");
        let result = jstime.run_script("date.day;", "jstime");
        assert_eq!(result.unwrap(), "13");
    }

    #[test]
    fn temporal_plain_time() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Temporal.PlainTime;", "jstime");
        assert_eq!(result.unwrap(), "function");
        let result = jstime.run_script(
            "const time = new Temporal.PlainTime(10, 30, 45); time.hour;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "10");
        let result = jstime.run_script("time.minute;", "jstime");
        assert_eq!(result.unwrap(), "30");
        let result = jstime.run_script("time.second;", "jstime");
        assert_eq!(result.unwrap(), "45");
    }

    #[test]
    fn temporal_plain_date_time() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Temporal.PlainDateTime;", "jstime");
        assert_eq!(result.unwrap(), "function");
        let result = jstime.run_script(
            "const dt = new Temporal.PlainDateTime(2025, 10, 13, 15, 30, 45); dt.year;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "2025");
        let result = jstime.run_script("dt.month;", "jstime");
        assert_eq!(result.unwrap(), "10");
        let result = jstime.run_script("dt.day;", "jstime");
        assert_eq!(result.unwrap(), "13");
        let result = jstime.run_script("dt.hour;", "jstime");
        assert_eq!(result.unwrap(), "15");
        let result = jstime.run_script("dt.minute;", "jstime");
        assert_eq!(result.unwrap(), "30");
    }

    #[test]
    fn temporal_instant_from() {
        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script("typeof Temporal.Instant.from;", "jstime");
        assert_eq!(result.unwrap(), "function");
        let result = jstime.run_script(
            "const instant = Temporal.Instant.from('2025-10-13T12:00:00Z'); typeof instant;",
            "jstime",
        );
        assert_eq!(result.unwrap(), "object");
    }
}
