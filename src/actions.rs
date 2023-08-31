//! Defines the actions that can be performed on the workspaces.

use super::state::State;
use super::workspace::Workspace;
use super::workspace::WorkspaceGroup;
use anyhow::Result;

/// Direction to move to. Used by switch_workspace.
#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Next,
    Prev,
}

/// Context of the current workspaces.
#[derive(Debug)]
pub struct WorkspaceContext {
    pub current_workspace: Workspace,
    pub workspaces: Vec<Workspace>,
    pub state: State,
}

/// Move to the next or previous workspace in the current group.
pub fn switch_workspace(
    context: &WorkspaceContext,
    direction: Direction,
) -> Result<Option<Workspace>> {
    let group = context.current_workspace.get_group()?;

    let mut group_workspaces = context
        .workspaces
        .iter()
        .filter(|x| {
            if let Ok(igroup) = x.get_group() {
                return igroup.char() == group.char();
            }

            false
        })
        .collect::<Vec<_>>();

    if direction == Direction::Prev {
        group_workspaces.reverse();
    }

    let mut group_workspaces_iter = group_workspaces.into_iter();
    group_workspaces_iter.find(|x| x.focused);

    let mut new_workspace: Option<Workspace> = None;

    if let Some(next_workspace) = group_workspaces_iter.next() {
        new_workspace = Some(next_workspace.clone());
    }

    Ok(new_workspace)
}

/// Toggle between both groups.
pub fn group_toggle(context: &WorkspaceContext) -> Result<Option<Workspace>> {
    let group = context.current_workspace.get_group()?;

    // If the current group is invalid, switch to the last focused valid group.
    let new_group = match group.is_valid() {
        true => group.toggle(),
        false => {
            let Some(ret) = WorkspaceGroup::from_id(context.state.last_group) else {
                return Ok(None);
            };

            ret
        }
    };

    let new_group_id = new_group.get_id()?;

    let last_used_workspace_id = match context.state.offset.get(&new_group_id) {
        Some(id) => *id,
        None => (new_group_id * 10) + 1,
    };

    let new_workspace = Workspace {
        name: last_used_workspace_id.to_string(),
        focused: true,
    };

    Ok(Some(new_workspace))
}

/// Move to other workspace by offset.
pub fn goto_workspace_offset(context: &WorkspaceContext, offset: u32) -> Result<Option<Workspace>> {
    let group = context.current_workspace.get_group()?;
    let group_id = group.get_id()?;

    let idx = match group.is_valid() {
        true => (group_id * 10) + offset,
        false => (context.state.last_group * 10) + offset,
    };

    let new_workspace = Workspace {
        name: idx.to_string(),
        focused: true,
    };

    Ok(Some(new_workspace))
}

/// Cycle through workspaces in the current group.
pub fn cycle(context: &WorkspaceContext) -> Result<Option<Workspace>> {
    let group = context.current_workspace.get_group()?;
    let group_id = group.get_id()?;

    if !group.is_valid() {
        return group_toggle(context);
    }

    let new_workspace_id = match context.state.cycle_offset.get(&group_id) {
        Some(id) => *id,
        None => (group_id * 10) + 1,
    };

    let new_workspace = Workspace {
        name: new_workspace_id.to_string(),
        focused: true,
    };

    Ok(Some(new_workspace))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn get_context(current_workspace_name: &String) -> WorkspaceContext {
        let workspaces = [11, 12, 14, 22, 25, 98, 99]
            .map(|x| Workspace {
                name: x.to_string(),
                focused: current_workspace_name == &x.to_string(),
            });

        WorkspaceContext {
            current_workspace: Workspace {
                name: current_workspace_name.to_string(),
                focused: true,
            },
            workspaces: workspaces.to_vec(),
            state: State {
                offset: HashMap::new(),
                cycle_offset: HashMap::new(),
                last_group: 1,
            }
        }
    }

    #[test]
    fn test_switch_workspace() -> Result<()> {
        let context = get_context(&"11".to_string());
        assert_eq!(switch_workspace(&context, Direction::Next)?.unwrap().name, "12");
        assert_eq!(switch_workspace(&context, Direction::Prev)?, None);

        let context = get_context(&"14".to_string());
        assert_eq!(switch_workspace(&context, Direction::Next)?, None);
        assert_eq!(switch_workspace(&context, Direction::Prev)?.unwrap().name, "12");

        let context = get_context(&"22".to_string());
        assert_eq!(switch_workspace(&context, Direction::Next)?.unwrap().name, "25");
        assert_eq!(switch_workspace(&context, Direction::Prev)?, None);

        Ok(())
    }
}
