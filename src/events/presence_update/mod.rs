use crate::{
    schema,
    structs::{
	FrameworkContext,
	Result,
    }
};

use poise::serenity_prelude as serenity;

use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    r2d2::Pool,
};

use self::serenity::{
    Context,
    Message,
    Presence
};

pub async fn presence_update(_ctx: &Context, _framework: FrameworkContext<'_>, presence: &Presence) -> Result<()> {
    println!("{}", presence.status.name());

    use diesel::select;
    use diesel::dsl::exists;
    use schema::status::dsl::*;
    let mut db = _framework.user_data.db_pool.get().expect("Couldn't get db connection from pool");
    let u_id = presence.user.id.as_u64().to_string();
    let stat = select(
	exists(
	    status
		.filter(user_id.eq(&u_id))
		.filter(guild_id.eq(presence.guild_id.unwrap().as_u64().to_string()))
	)
    )
	.get_result(&mut db)
	.expect("Error checking if address exist");
    if stat {
	diesel::delete(
	    status
		.filter(user_id.eq(&u_id))
		.filter(guild_id.eq(presence.guild_id.unwrap().as_u64().to_string()))
	)
	    .execute(&mut db)
	    .unwrap();
	diesel::insert_into(status)
	    .values((user_id.eq(&u_id), guild_id.eq(presence.guild_id.unwrap().as_u64().to_string())))
	    .execute(&mut db)
	    .expect("Error saving new balance");
    } else {
	diesel::insert_into(status)
	    .values((user_id.eq(&u_id), guild_id.eq(presence.guild_id.unwrap().as_u64().to_string())))
	    .execute(&mut db)
	    .expect("Error saving new balance");
    }


    Ok(())
}
