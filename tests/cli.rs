use assert_cmd::Command;

#[test]
fn runs() {
    // this should fail because arguments are missing
    let mut cmd = Command::cargo_bin("smartcontract_bootstrapper").unwrap();
    cmd.assert().failure();
}
