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
pub async fn address(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
    #[description = "Address"] email_address: String,
) -> CommandResult {
    
    let mut db = ctx.data().db_pool.get().expect("Couldn't get db connection from pool");
    use diesel::select;
    use diesel::dsl::exists;
    use schema::options::dsl::*;
    let u_id = ctx.author().id.as_u64().to_string();

    let opt = select(
	exists(
	    options
		.filter(user_id.eq(&u_id))
		.filter(guild_id.eq(ctx.guild().unwrap().id.as_u64().to_string()))
		.filter(option.eq("contact.email.address"))
	)
    )
	.get_result(&mut db)
	.expect("Error checking if address exist");
    if opt {
	diesel::update(
	    options
		.filter(user_id.eq(&u_id))
		.filter(guild_id.eq(ctx.guild().unwrap().id.as_u64().to_string()))
		.filter(option.eq("contact.email.address"))
	)
	    .set(value.eq(&email_address))
	    .execute(&mut db)
	    .unwrap();
    } else {
	diesel::insert_into(options)
	    .values((user_id.eq(&u_id), guild_id.eq(ctx.guild().unwrap().id.as_u64().to_string()), option.eq("contact.email.address"), value.eq(&email_address)))
	    .execute(&mut db)
	    .expect("Error saving new balance");
    }

    ctx.send(|f| f
	     .content(format!("Set your email address to {}.", email_address))
	     .ephemeral(true) // this one only applies in application commands though
    ).await?;
    Ok(())
}
