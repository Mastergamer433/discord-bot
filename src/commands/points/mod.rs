mod subcommands;

use crate::structs::Context;
use crate::structs::Command;
use crate::structs::CommandResult;
use subcommands::*;

#[poise::command(prefix_command, track_edits, slash_command, subcommands("balance", "gift"))]
pub async fn points(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> CommandResult {
    
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [points()]
}
