use std::process::Command;

#[test]
fn test_basic_usage_example() {
    let output = Command::new("cargo")
        .args(&["test", "--example", "basic_no_disc"])
        .output()
        .expect("Failed to execute example");

    assert!(output.status.success());
}

#[test]
fn test_basic_usage_example2() {
    let output = Command::new("cargo")
        .args(&["test", "--example", "basic_disc"])
        .output()
        .expect("Failed to execute example");

    assert!(output.status.success());
}
