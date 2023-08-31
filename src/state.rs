//! Application state written as JSON to a file.

use super::xdg;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    /// Offset of last selected workspaces in each group.
    /// Key is group ID, value is workspace ID.
    pub offset: HashMap<u32, u32>,

    /// Offset of the workspace to navigate to when using the cycle action.
    /// Key is group ID, value is workspace ID.
    pub cycle_offset: HashMap<u32, u32>,

    /// The last valid group that was used.
    pub last_group: u32,
}

/// Get path to state file.
pub fn get_state_file() -> Result<PathBuf> {
    let mut path = xdg::get_app_state_dir()?;
    path.push("state.json");
    Ok(path)
}

/// Read state from file.
fn read_state() -> Result<State> {
    let state_file = get_state_file()?;
    let json_str = fs::read_to_string(state_file)?;
    let ret = serde_json::from_str(json_str.as_str())?;

    Ok(ret)
}

/// Write state from file.
pub fn write_state(state: &State) -> Result<()> {
    let state_file = get_state_file()?;
    let json_state = serde_json::to_string(&state)?;
    fs::write(state_file, json_state)?;
    Ok(())
}

/// Read state from file or return default empty state.
pub fn read_init_state() -> State {
    read_state().unwrap_or(State {
        offset: HashMap::new(),
        cycle_offset: HashMap::new(),
        last_group: 1,
    })
}
