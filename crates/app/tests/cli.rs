//! End-to-end test: run the real binary.

use std::process::Command;

fn essential() -> Command {
    Command::new(env!("CARGO_BIN_EXE_essential"))
}

#[test]
fn greets() {
    let output = essential()
        .args(["greet", "Ada"])
        .env("APP_NAME", "Demo")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Hello, Ada! Welcome to Demo."
    );
}

#[test]
fn blank_name_fails() {
    let output = essential().args(["greet", " "]).output().unwrap();
    assert!(!output.status.success());
}

#[test]
fn todo_demo_runs() {
    let output = essential().arg("todo").output().unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("[x] 1 ship the template"));
}
