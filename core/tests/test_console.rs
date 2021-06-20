use jstime_core as jstime;

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
        jstime::init(None);
        let options = jstime::Options::default();
        
        let mut jstime = jstime::JSTime::new(options);
        let result = jstime.run_script(command, "filename");
        assert_eq!(result.unwrap(), "undefined");
        let output = String::new();
        output
    }
}