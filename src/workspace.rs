//! For testability and convenience we define types for Workspace and WorkspaceGroup instead of
//! using swayipc::Workspace directly.

use anyhow::Context;
use anyhow::Result;
use swayipc::Workspace as SwayWorkspace;

#[derive(Debug, PartialEq, Clone)]
pub struct Workspace {
    pub name: String,
    pub focused: bool,
}

/// Construct a Workspace from a swayipc::Workspace.
impl From<&SwayWorkspace> for Workspace {
    fn from(sway_workspace: &SwayWorkspace) -> Self {
        Workspace {
            name: sway_workspace.name.clone(),
            focused: sway_workspace.focused,
        }
    }
}

impl Workspace {
    /// Return the workspace name as numeric representation.
    /// Return None if the workspace name is not numeric.
    pub fn get_id(&self) -> Result<u32> {
        self.name
            .parse::<u32>()
            .with_context(|| format!("Unable to get ID of workspace group {}", self.name))
    }

    /// Return the workspace group, if it can be determined.
    pub fn get_group(&self) -> Result<WorkspaceGroup> {
        WorkspaceGroup::from_workspace(self).context("Failed to get workspace group")
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WorkspaceGroup(char);

impl WorkspaceGroup {
    /// Return the workspace group, if it can be determined.
    pub fn from_workspace(workspace: &Workspace) -> Option<WorkspaceGroup> {
        let sway_workspace_name = &workspace.name;

        let Some(c) = sway_workspace_name.chars().next() else {
            return None;
        };

        Some(WorkspaceGroup(c))
    }

    /// Construct WorkspaceGroup based on group ID.
    pub fn from_id(id: u32) -> Option<WorkspaceGroup> {
        let name = id.to_string();

        let Some(c) = name.chars().next() else {
            return None;
        };

        Some(WorkspaceGroup(c))
    }

    /// Returns true if this is a valid workspace group we care about.
    /// Valid groups are 1 and 2.
    pub fn is_valid(&self) -> bool {
        let Ok(id) = self.get_id() else {
            return false;
        };

        (1..=2).contains(&id)
    }

    /// Returns the workspace group as numeric representation, if possible.
    pub fn get_id(&self) -> Result<u32> {
        self.0
            .to_digit(10)
            .with_context(|| format!("Unable to get ID of workspace group {}", self.0))
    }

    /// Workspace representation as char. It's the first char of the workspace name.
    pub fn char(&self) -> char {
        self.0
    }

    /// Toggle between both groups.
    pub fn toggle(&self) -> WorkspaceGroup {
        let id = match self.0 {
            '1' => '2',
            _ => '1',
        };

        WorkspaceGroup(id)
    }
}
