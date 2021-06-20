use jstime_core as jstime;

use std::io::Read;
use gag::BufferRedirect;

mod common;

#[cfg(test)]
mod console {
    use super::*;

    #[test]
    fn without_real_specifier_word_start() {
        let result = read_from_console("console.log('first', '%second', 'third')");
        assert_eq!(&result[..], "first %second third\n");
    }

    #[test]
    fn without_real_specifier_word_end() {
        let result = read_from_console("console.log('first%s', 'second', 'third')");
        assert_eq!(&result[..], "first%s second third\n");
    }

    fn read_from_console(command: &str) -> String {
        let mut buf = BufferRedirect::stdout().unwrap();

        let _setup_guard = common::setup();
        let options = jstime::Options::default();
        let mut jstime = jstime::JSTime::new(options);
        jstime.run_script(command, "jstime").unwrap();

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);
        output
    }
}
