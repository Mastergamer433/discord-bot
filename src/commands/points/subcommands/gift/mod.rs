use poise::serenity_prelude::User;

use crate::{
    schema,
    structs::{
	CommandResult,
	Context
    }, models::Balance
};


use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::ConnectionManager,
    r2d2::Pool,
};
use poise::serenity_prelude as serenity;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn gift(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "User"] user: User,
    #[description = "Amount"] amount: f32,
) -> CommandResult {
    
    let mut db = ctx.data().db_pool.get().expect("Couldn't get db connection from pool");

    use diesel::select;
    use diesel::dsl::exists;

    let balance = select(exists(schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(ctx.author().id.as_u64().to_string()))))
	.get_result(&mut db)
	.expect("Error checking if balance exist");
    if balance {
	let balance: Balance = schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(ctx.author().id.as_u64().to_string())).get_result(&mut db).expect("Error getting balance");
	if balance.points < amount {
	    println!("Sender doesn't have enough points");
	    ctx.send(|f| f
		     .content("You do not have enough point for this transaction...")
		     .ephemeral(true) // this one only applies in application commands though
	    ).await?;
	    return Ok(());
	} else {
	    let user_id: &str = &user.id.as_u64().to_string();
	    let balance = select(exists(schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(user_id))))
		.get_result(&mut db)
		.expect("Error checking if balance exist");
	    if balance {
		diesel::update(schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(user_id)))
		    .set(schema::balance::points.eq(schema::balance::points+amount))
		    .execute(&mut db)
		    .unwrap();
	    } else {
		diesel::insert_into(schema::balance::dsl::balance)
		    .values((schema::balance::dsl::user_id.eq(user_id), schema::balance::dsl::points.eq(amount)))
		    .execute(&mut db)
		    .expect("Error saving new post");
	    }
	    diesel::update(schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(ctx.author().id.as_u64().to_string())))
		.set(schema::balance::points.eq(schema::balance::points-amount))
		.execute(&mut db)
		.unwrap();
	}
    } else {
	println!("Sender doesn't have a balance");
	ctx.send(|f| f
		 .content("You do not have a balance...")
		 .ephemeral(true) // this one only applies in application commands though
	).await?;
	return Ok(());
    }

    ctx.send(|f| f
	     .content(format!("Sent {} points to <@{}>!", amount, user.id.as_u64()))
	     .ephemeral(true) // this one only applies in application commands though
    ).await?;
    Ok(())
}
