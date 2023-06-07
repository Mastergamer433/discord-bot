mod subcommands;

use crate::structs::Context;
use crate::structs::Command;
use crate::structs::CommandResult;
use subcommands::email::email;

#[poise::command(prefix_command, track_edits, slash_command, subcommands("email"))]
pub async fn options(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<i32>,
) -> CommandResult {
    
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [options()]
}
