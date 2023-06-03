#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn gift(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
    #[description = "User"] choice: User,
    #[description = "Amount"] choice: String,
) -> CommandResult {
    
    Ok(())
}
