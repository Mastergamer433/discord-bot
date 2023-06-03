use poise::serenity_prelude::User;

use crate::{
    schema,
    structs::{
	CommandResult,
	Context,
    },
    models::Balance,
};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn balance(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
    #[description = "User"] user: Option<User>,
) -> CommandResult {
    
    // let mut db = ctx.data().db_pool.get().expect("Couldn't get db connection from pool");
    // use diesel::select;
    // use diesel::dsl::exists;

    // let user_id = if let Some(user) = user { user.id.as_u64().to_string() } else { ctx.author().id.as_u64().to_string() };
    // let balance = select(exists(schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(user_id))))
    // 	.get_result(&mut db)
    // 	.expect("Error checking if balance exist");
    // if balance {

    // 	let balance: Balance = schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(ctx.author().id.as_u64().to_string())).get_result(&mut db).expect("Error getting balance");
    // } else {
    // 	println!("User doesn't have a balance");
    // 	ctx.send(|f| f
    // 		 .content("You do not have a balance...")
    // 		 .ephemeral(true) // this one only applies in application commands though
    // 	).await?;
    // 	return Ok(());
    // }
    Ok(())
}
