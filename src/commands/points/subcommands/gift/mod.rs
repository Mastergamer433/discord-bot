use poise::serenity_prelude::User;

use crate::{
    structs::{
	CommandResult,
	Context
    },
    models::Balance,
    helpers::transfer
};


use poise::serenity_prelude as serenity;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn gift(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "User"] user: User,
    #[description = "Amount"] amount: f32,
) -> CommandResult {
    let receiver_user_id = user.id.as_u64().to_string();
    let sender_user_id = &ctx.author().id.as_u64().to_string();
    let transfer = transfer(ctx.data().db_pool.get().expect("Could not get database"), &sender_user_id, &receiver_user_id, amount).await;
    Ok(())
}
