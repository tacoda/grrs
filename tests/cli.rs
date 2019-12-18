use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use tempfile::NamedTempFile;
use std::io::{self, Write};

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("tacoda_grrs")?;
    cmd.arg("foobar")
        .arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("tacoda_grrs")?;
    cmd.arg("test")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

// What to test?

// While it can certainly be fun to write integration tests, it will also take some time to write them, as well as to update them when your application’s behavior changes. To make sure you use your time wisely, you should ask yourself what you should test.

// In general it’s a good idea to write integration tests for all types of behavior that a user can observe. That means that you don’t need to cover all edge cases: It usually suffices to have examples for the different types and rely on unit tests to cover the edge cases.

// It is also a good idea not to focus your tests on things you can’t actively control. It would be a bad idea to test the exact layout of --help as it is generated for you. Instead, you might just want to check that certain elements are present.

// Depending on the nature of your program, you can also try to add more testing techniques. For example, if you have extracted parts of your program and find yourself writing a lot of example cases as unit tests while trying to come up with all the edge cases, your should look into proptest. If you have a program which consumes arbitrary files and parses them, try to write a fuzzer to find bugs in edge cases.
