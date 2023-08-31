mod actions;
mod app;
mod state;
mod workspace;
mod xdg;

use anyhow::Result;
use app::Action;
use app::App;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::process::exit;
use actions::Direction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    current_workspace_id: u32,
    current_group_id: u32,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Switch to the previous workspace with the current group.
    Prev,
    /// Switch to the next workspace with the current group.
    Next,
    /// Toggle between both groups.
    ToggleGroup,
    /// Goto workspace offset within current group.
    Goto { offset: u32 },
    /// Move current container to workspace offset within current group.
    /// If offset is not specified, toggle between both groups.
    Move { offset: Option<u32> },
    /// Cycle through all workspaces within current group.
    Cycle,
    /// Save currently selected workspace to state.
    Save,
    /// Show information.
    Info,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut app = App::new(cli.debug, cli.dry_run)?;

    let context = app.get_workspace_context()?;

    let action: Option<Action> = match &cli.command {
        Some(Commands::Next {}) | Some(Commands::Prev {}) => {
            let direction = match &cli.command {
                Some(Commands::Next {}) => Direction::Next,
                Some(Commands::Prev {}) => Direction::Prev,
                _ => unreachable!(),
            };

            Some(Action {
                new_workspace: actions::switch_workspace(&context, direction)?,
                move_container: false,
            })
        }
        Some(Commands::ToggleGroup {}) => Some(Action {
            new_workspace: actions::group_toggle(&context)?,
            move_container: false,
        }),
        Some(Commands::Goto { offset }) => Some(Action {
            new_workspace: actions::goto_workspace_offset(&context, *offset)?,
            move_container: false,
        }),
        Some(Commands::Move { offset }) => match offset {
            Some(offset) => Some(Action {
                new_workspace: actions::goto_workspace_offset(&context, *offset)?,
                move_container: true,
            }),
            None => Some(Action {
                new_workspace: actions::group_toggle(&context)?,
                move_container: true,
            }),
        },
        Some(Commands::Cycle {}) => Some(Action {
            new_workspace: actions::cycle(&context)?,
            move_container: false,
        }),
        Some(Commands::Save {}) => Some(Action {
            new_workspace: None,
            move_container: false,
        }),
        Some(Commands::Info {}) => {
            let group = context.current_workspace.get_group()?;
            let group_id = group.get_id()?;
            let current_workspace_id = context.current_workspace.get_id()?;

            let info = Info {
                current_workspace_id,
                current_group_id: group_id,
            };

            let json = serde_json::to_string(&info)?;
            println!("{}", json);

            None
        }
        None => {
            eprintln!("Command expected");
            exit(1)
        }
    };

    match action {
        Some(action) => app.commit_action(&action, &context),
        _ => Ok(()),
    }
}
