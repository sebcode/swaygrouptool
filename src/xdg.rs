//! Helper functions to access the XDG state directory.

use anyhow::Result;
use std::{fs::create_dir_all, path::PathBuf};

/// Get path to XDG state directory. Usually ~/.local/state.
pub fn get_xdg_state_dir() -> Result<PathBuf> {
    if let Ok(xdg_data_home) = std::env::var("XDG_STATE_HOME") {
        return Ok(PathBuf::from(xdg_data_home));
    }

    let home = std::env::var("HOME")?;
    let path_name_str = format!("{}/.local/state", home);
    let path = PathBuf::from(path_name_str);
    Ok(path)
}

/// Get path to XDG state directory for this app. Usually ~/.local/state/swaygrouptool.
/// Also creates the directory if it does not exist.
pub fn get_app_state_dir() -> Result<PathBuf> {
    let current_exe = std::env::current_exe()?;
    let file_name = current_exe.file_name().unwrap();

    let mut dir = get_xdg_state_dir()?;
    dir.push(file_name);
    create_dir_all(&dir)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::{env, fs::remove_dir};

    use super::*;

    #[test]
    #[serial]
    fn test_get_xdg_state_dir() -> Result<()> {
        env::set_var("XDG_STATE_HOME", "/tmp/asdf");
        env::set_var("HOME", "/tmp/fakehome");

        assert_eq!("/tmp/asdf", get_xdg_state_dir()?.to_str().unwrap());

        env::remove_var("XDG_STATE_HOME");

        assert_eq!(
            "/tmp/fakehome/.local/state",
            get_xdg_state_dir()?.to_str().unwrap()
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn test_get_app_state_dir() -> Result<()> {
        let current_exe = std::env::current_exe()?;
        let binary_name = current_exe.file_name().unwrap();

        let mut fake_state_dir = env::temp_dir();
        env::set_var("XDG_STATE_HOME", &fake_state_dir);
        fake_state_dir.push(binary_name);
        let _ = remove_dir(&fake_state_dir);

        assert!(!fake_state_dir.exists());
        assert_eq!(
            fake_state_dir.to_str().unwrap(),
            get_app_state_dir()?.to_str().unwrap()
        );
        assert!(fake_state_dir.exists());

        let _ = remove_dir(&fake_state_dir);
        assert!(!fake_state_dir.exists());

        Ok(())
    }
}
