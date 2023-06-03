#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn balance(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
    #[description = "User"] choice: Option<String>,
) -> CommandResult {
    
    Ok(())
}
