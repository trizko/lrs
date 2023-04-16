use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn outputs_non_hidden_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lrs")?;

    cmd.arg("./tests/test_dir/");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test_file.txt"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(".hidden_test_file").not());

    Ok(())
}

#[test]
fn outputs_hidden_files_with_all_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lrs")?;

    cmd.arg("-a").arg("./tests/test_dir/");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test_file.txt"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(".hidden_test_file"));

    Ok(())
}

#[test]
fn outputs_filenames_with_newlines_with_list_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lrs")?;

    cmd.arg("-la").arg("./tests/test_dir/");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test_file.txt\n"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(".hidden_test_file\n"));

    Ok(())
}

#[test]
fn outputs_to_stderr_when_file_or_dir_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lrs")?;

    cmd.arg("./dir_not_exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}
