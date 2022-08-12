#!/bin/bash
# This script downloads and sets up a rust project for a kattis problem.
# Note: It is not guaranteed to work for all projects and some modifications might be needed.
# Usage:
# ./dl_problem.sh different
# ./dl_problem.sh dice_cup
set -xe

problem_name=$1
tests_dir=$problem_name/tests
samples_dir=$tests_dir/samples

# remove underscores for kattis url
kattis_name=${problem_name//_/}

# starting a rust project with digits is not allowed.
if [[ ${problem_name::1} == [0-9] ]]; then
    problem_name="rs_"$problem_name
fi

# create project
cargo new --vcs=none $problem_name
mkdir $tests_dir

# download samples and remove artifacts
wget https://open.kattis.com/problems/$kattis_name/file/statement/samples.zip && unzip samples.zip -d $samples_dir && rm samples.zip

# rename golden files to be arbitrarily numbered
find $samples_dir/*.in | cat -n | while read n f; do mv -n "$f" "$samples_dir/$n.in"; done
find $samples_dir/*.ans | cat -n | while read n f; do mv -n "$f" "$samples_dir/$n.ans"; done

# create integration tests for samples
touch $tests_dir/sample_test.rs
cat > $tests_dir/sample_test.rs <<EOF
use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn solve(input_file: &str, output_file: &str) -> TestResult {
    let expected = fs::read_to_string(output_file)?;
    Command::cargo_bin("$problem_name")?
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
EOF

# add dependencies for the integration tests
cat >> $problem_name/Cargo.toml <<- "EOF"
[dev-dependencies]
assert_cmd = "2.0.4"
EOF
