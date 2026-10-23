use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn bgs_command(bgs_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(bgs_utils_cargo_bin::cargo_bin("bgs")?);
    cmd.env("BGS_HOME", bgs_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let bgs_home = TempDir::new()?;

    bgs_command(bgs_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`bgs update` is not available in debug builds"));

    Ok(())
}
