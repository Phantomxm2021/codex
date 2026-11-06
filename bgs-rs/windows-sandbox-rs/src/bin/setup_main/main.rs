#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    bgs_windows_sandbox::setup_helper_main()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("bgs-windows-sandbox-setup is Windows-only");
}
