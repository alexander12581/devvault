use std::fs;
use std::path::PathBuf;

use crate::error::DevVaultError;

pub fn install() -> Result<(), DevVaultError> {
    let hooks_dir = PathBuf::from(".git").join("hooks");
    if !hooks_dir.exists() {
        return Err(DevVaultError::Other(anyhow::anyhow!(
            "Not a git repository. No .git/hooks directory found."
        )));
    }
    let hook_path = hooks_dir.join("pre-commit");
    let hook_content = if cfg!(target_os = "windows") {
        include_str!("../hooks/pre-commit.bat")
    } else {
        include_str!("../hooks/pre-commit.sh")
    };
    fs::write(&hook_path, hook_content)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms)?;
    }
    Ok(())
}

pub fn uninstall() -> Result<(), DevVaultError> {
    let hook_path = PathBuf::from(".git").join("hooks").join("pre-commit");
    if hook_path.exists() {
        fs::remove_file(&hook_path)?;
    }
    Ok(())
}