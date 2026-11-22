use assert_cmd::Command;

#[test]
fn test_cli_help_and_scan_stub() {
    let mut help_cmd = Command::cargo_bin("audit-triage").unwrap();
    help_cmd.arg("--help").assert().success();

    let mut scan_cmd = Command::cargo_bin("audit-triage").unwrap();
    scan_cmd.args(["scan", "."]).assert().code(2);
}
