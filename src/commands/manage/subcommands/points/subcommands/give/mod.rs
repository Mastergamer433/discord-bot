use crate::{
    schema,
    models::Balance,
    structs::{
	CommandResult,
	Context,
    }, helpers::check_permission,
};

use anyhow::anyhow;
use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    r2d2::Pool,
};
use poise::serenity_prelude as serenity;
use self::serenity::User;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn give(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
    #[description = "User"] user: User,
    #[description = "Amount"] amount: f32,
) -> CommandResult {
    
    let mut db = ctx.data().db_pool.get().expect("Couldn't get db connection from pool");
    let has_perms = check_permission(ctx.data().db_pool.get().expect("Couldn't get db connection from pool"), ctx.guild_id().unwrap(), ctx.author().id, "CommandManagePointsGive").await.expect("Couldn't check permissions due to error...");
    if !has_perms {
	ctx.send(|f| f
		 .content("You do not have permissions to do that...")
		 .ephemeral(true) // this one only applies in application commands though
	).await?;
	return Err(anyhow!("Missing permissions..."));
    };
    use diesel::select;
    use diesel::dsl::exists;
    use schema::balance::dsl::*;
    let u_id = user.id.as_u64().to_string();

    let bal = select(exists(balance.filter(user_id.eq(&u_id))))
	.get_result(&mut db)
	.expect("Error checking if balance exist");
    if bal {

	let bal: Balance = balance.filter(user_id.eq(&u_id)).get_result(&mut db).expect("Error getting balance");

	diesel::update(balance.filter(user_id.eq(&u_id)))
	    .set(points.eq(points+amount))
	    .execute(&mut db)
	    .unwrap();
    } else {
	diesel::insert_into(balance)
	    .values((user_id.eq(&u_id), points.eq(amount)))
	    .execute(&mut db)
	    .expect("Error saving new balance");
    }

    ctx.send(|f| f
	     .content(format!("Gave <@{}> {} points.", u_id, amount))
	     .ephemeral(true) // this one only applies in application commands though
    ).await?;
    Ok(())
}
