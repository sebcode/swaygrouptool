use anyhow::Result;

use super::actions::WorkspaceContext;
use super::state;
use super::workspace::Workspace;
use anyhow::Context;

#[derive(Debug)]
pub struct Action {
    pub new_workspace: Option<Workspace>,
    pub move_container: bool,
}

pub struct App {
    connection: Option<swayipc::Connection>,
    debug: bool,
    dry_run: bool,
}

impl App {
    pub fn new(debug: bool, dry_run: bool) -> Result<App> {
        Ok(App {
            connection: None,
            debug,
            dry_run,
        })
    }

    pub fn get_connection(&mut self) -> Result<&mut swayipc::Connection> {
        match self.connection {
            Some(ref mut conn) => Ok(conn),
            None => {
                let sway_connection =
                    swayipc::Connection::new().context("Unable to connect to sway sock")?;
                self.connection = Some(sway_connection);
                self.get_connection()
            }
        }
    }

    pub fn get_workspace_context(&mut self) -> Result<WorkspaceContext> {
        let sway_connection = self.get_connection()?;

        let sway_workspaces = sway_connection
            .get_workspaces()
            .context("Failed to fetch workspaces")?;

        let workspaces: Vec<Workspace> = sway_workspaces
            .iter()
            .map(Workspace::from)
            .collect::<Vec<_>>();

        let current_workspace = workspaces
            .iter()
            .find(|x| x.focused)
            .context("Cannot find focused workspace")?;

        let state = state::read_init_state();

        Ok(WorkspaceContext {
            current_workspace: current_workspace.clone(),
            workspaces,
            state,
        })
    }

    pub fn commit_action(&mut self, action: &Action, context: &WorkspaceContext) -> Result<()> {
        let is_debug = self.debug;
        let is_dry = self.dry_run;

        if is_debug {
            println!("ACTION {:?}", action);
        }

        let Some(new_workspace) = &action.new_workspace else {
            return Ok(());
        };

        let current_group = context.current_workspace.get_group()?;
        let current_group_id = current_group.get_id()?;
        let current_workspace_id = context.current_workspace.get_id()?;

        let new_group = new_workspace.get_group()?;
        let new_group_id = new_group.get_id()?;
        let new_workspace_id = new_workspace.get_id()?;

        let mut new_state = context.state.clone();

        let connection = self.get_connection()?;

        if is_debug {
            println!("commit: goto_workspace {}", new_workspace.name);
        }

        if action.move_container {
            connection.run_command(&format!(
                "move container to workspace {}",
                &new_workspace.name
            ))?;
        }

        if !is_dry {
            connection.run_command(&format!("workspace {}", &new_workspace.name))?;
        }

        let did_change =
            new_group_id != current_group_id || current_workspace_id != new_workspace_id;

        if !did_change {
            return Ok(());
        }

        new_state.offset.insert(new_group_id, new_workspace_id);

        if new_group.is_valid() {
            new_state.last_group = new_group_id;
        }

        println!("insert offset {} {}", new_group_id, new_workspace_id);

        if new_group_id == current_group_id {
            println!("new_group_id == current_group_id");

            new_state
                .cycle_offset
                .insert(current_group_id, current_workspace_id);
            println!("insert cycle {} {}", current_group_id, current_workspace_id);
        } else {
            println!("new_group_id != current_group_id");
        }

        if !is_dry {
            state::write_state(&new_state)?;
        }

        if is_debug {
            println!("new state {:?}", new_state);
        }

        Ok(())
    }
}
