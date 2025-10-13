use assert_cmd::prelude::*;

use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("-h")
        .assert()
        .stdout(predicate::str::contains("jstime [OPTIONS] [filename]"))
        .success()
        .code(0);
}

#[test]
fn entry_no_deps() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/queue-microtask.js")
        .assert()
        .stdout("0\n1\n2\n3\n4\n5\n")
        .success()
        .code(0);
}

#[test]
fn entry_with_deps() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/module.mjs")
        .assert()
        .stdout("This should only be logged once.\nhello world exactly 1 time\n")
        .success()
        .code(0);
}

#[test]
fn call_to_function_that_does_not_exist() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/function-does-not-exist.js")
        .assert()
        .stderr(predicate::str::contains(
            "ReferenceError: fhqwhgads is not defined",
        ))
        .failure()
        .code(1);
}

#[test]
fn throw() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/throw.js")
        .assert()
        .stderr(predicate::str::contains("Error: oh no"))
        .failure()
        .code(1);
}

#[test]
fn invalid_code() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/invalid-code.js")
        .assert()
        .stderr(predicate::str::contains(
            "SyntaxError: Unexpected token '}'",
        ))
        .failure()
        .code(1);
}

#[test]
fn console() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/console-printer.js")
        .assert()
        .stdout(
            r#"first %second third
first%s second third
first second third
first second 3
first second %s
"#,
        )
        .code(0);
}

#[test]
fn top_level_await() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/top-level-await.js")
        .assert()
        .stdout("0\n1\n2\n")
        .success()
        .code(0);
}

#[test]
fn module_graph_execution_order() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/module-graph-exec-order/root.mjs")
        .assert()
        .stdout(
            r#"start b
promise b
start c
promise c
microtask b
microtask c
end b
end c
start a
b
promise a
microtask a
end a
start root
a
c
end root
"#,
        )
        .success()
        .code(0);
}

#[test]
fn temporal() {
    Command::cargo_bin("jstime")
        .unwrap()
        .arg("./tests/fixtures/temporal.js")
        .assert()
        .stdout("Date: 2025 10 13\nTime: 15 30 45\nDateTime: 2025 10 13 15 30\n")
        .success()
        .code(0);
}
