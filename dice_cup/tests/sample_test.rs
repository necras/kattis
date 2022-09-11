use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn solve(input_file: &str, output_file: &str) -> TestResult {
    let expected = fs::read_to_string(output_file)?;
    Command::cargo_bin("dice_cup")?
        .pipe_stdin(input_file)?
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn sample01() -> TestResult {
    solve("tests/samples/1.in", "tests/samples/1.ans")
}

#[test]
fn sample02() -> TestResult {
    solve("tests/samples/2.in", "tests/samples/2.ans")
}

#[test]
fn sample03() -> TestResult {
    solve("tests/samples/3.in", "tests/samples/3.ans")
}
