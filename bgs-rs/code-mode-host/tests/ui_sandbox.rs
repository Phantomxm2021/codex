#[test]
fn plugin_arguments_are_parsed_before_startup() -> Result<(), Box<dyn std::error::Error>> {
    let program = bgs_utils_cargo_bin::cargo_bin("bgs-code-mode-host")?;
    for arguments in [
        ["--help", "--bgs-ui-plugin-process"],
        ["--bgs-ui-plugin-process", "--help"],
    ] {
        let output = bgs_utils_process::background_command(&program)
            .args(arguments)
            .output()?;
        assert!(output.status.success());
        assert!(String::from_utf8(output.stdout)?.contains("Usage:"));
    }
    for arguments in [
        ["--unknown", "--bgs-ui-plugin-process"],
        ["--bgs-ui-plugin-process", "--unknown"],
        ["--listen=stdio", "--bgs-ui-plugin-process"],
        ["--bgs-ui-plugin-process", "--listen=stdio"],
    ] {
        let output = bgs_utils_process::background_command(&program)
            .args(arguments)
            .output()?;
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8(output.stderr)?.contains("error:"));
    }
    Ok(())
}
