//! Resolves local sandbox implementations and checks support for an explicitly selected remote MXC.

use bgs_file_system::FileSystemSandboxContext;
use bgs_file_system::WindowsSandboxSelection;
use bgs_protocol::config_types::WindowsSandboxLevel;
use bgs_protocol::models::PermissionProfile;
use bgs_sandboxing::SandboxManager;
use bgs_sandboxing::SandboxType;
use bgs_sandboxing::SandboxablePreference;

pub(crate) fn validate_remote_mxc(info: &crate::EnvironmentInfo) -> std::io::Result<()> {
    if !info.capabilities.windows_mxc {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "exec-server does not support the requested MXC Windows sandbox; update the executor",
        ));
    }
    Ok(())
}

pub(crate) fn select_sandbox(
    manager: &SandboxManager,
    permission_profile: &PermissionProfile,
    sandbox_context: &FileSystemSandboxContext,
    has_managed_network_requirements: bool,
) -> (SandboxType, Option<WindowsSandboxLevel>) {
    let windows_sandbox_level = match sandbox_context.windows_sandbox_selection {
        WindowsSandboxSelection::Disabled => WindowsSandboxLevel::Disabled,
        WindowsSandboxSelection::RestrictedToken => WindowsSandboxLevel::RestrictedToken,
        WindowsSandboxSelection::Elevated => WindowsSandboxLevel::Elevated,
        WindowsSandboxSelection::Mxc => return (SandboxType::WindowsMxc, None),
    };
    let windows_sandbox_type = match windows_sandbox_level {
        WindowsSandboxLevel::Disabled => SandboxType::None,
        WindowsSandboxLevel::RestrictedToken | WindowsSandboxLevel::Elevated => {
            SandboxType::WindowsRestrictedToken
        }
    };
    let sandbox_type = manager.select_initial(
        permission_profile,
        SandboxablePreference::Require,
        windows_sandbox_type,
        has_managed_network_requirements,
    );
    (sandbox_type, Some(windows_sandbox_level))
}
