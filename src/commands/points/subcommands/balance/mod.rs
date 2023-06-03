use crate::{
    schema,
    models::Balance,
    structs::{
	CommandResult,
	Context,
    },
};

use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    r2d2::Pool,
};
use poise::serenity_prelude as serenity;
use self::serenity::User;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn balance(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
    #[description = "User"] user: Option<User>,
) -> CommandResult {
    
    let mut db = ctx.data().db_pool.get().expect("Couldn't get db connection from pool");
    use diesel::select;
    use diesel::dsl::exists;
    use schema::balance::dsl::*;

    let target_user_id = if let Some(user) = user { let u_id = user.id.as_u64().to_string(); u_id } else { let u_id = ctx.author().id.as_u64().to_string(); u_id };
    let bal = select(exists(balance.filter(user_id.eq(&target_user_id))))
	.get_result(&mut db)
	.expect("Error checking if balance exist");
    if bal {

	let bal: Balance = balance.filter(user_id.eq(&target_user_id)).get_result(&mut db).expect("Error getting balance");

	ctx.send(|f| f
		 .content(format!("<@{}> have {} points.", target_user_id, bal.points))
		 .ephemeral(true) // this one only applies in application commands though
	).await?;
    } else {
	println!("User doesn't have a balance");
	ctx.send(|f| f
		 .content("You do not have a balance...")
		 .ephemeral(true) // this one only applies in application commands though
	).await?;
	return Ok(());
    }
    Ok(())
}
