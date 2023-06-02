use crate::{
    schema,
    structs::{
	FrameworkContext,
	Result,
    }
};
use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::ConnectionManager,
    r2d2::Pool,
};
use poise::serenity_prelude as serenity;
use self::serenity::{
    Context,
    Message,
};

pub async fn message(_ctx: &Context, _framework: FrameworkContext<'_>, new_message: &Message) -> Result<()> {

    println!("Author: {}, Content: {}", new_message.author.name, new_message.content);
    let mut db = _framework.user_data.db_pool.get().expect("Couldn't get db connection from pool");

    let points: &f32 = &(new_message.content.chars().count() as f32 * 0.06);
    let user_id: u64 = *new_message.author.id.as_u64();
    let user_id: &str = &user_id.to_string();
    use diesel::select;
    use diesel::dsl::exists;

    let balance = select(exists(schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(user_id))))
	.get_result(&mut db)
	.expect("Error checking if balance exist");

    if balance {
	diesel::update(schema::balance::dsl::balance.filter(schema::balance::dsl::user_id.eq(user_id)))
	    .set(schema::balance::points.eq(schema::balance::points+points))
	    .execute(&mut db)
	    .unwrap();
    } else {
	diesel::insert_into(schema::balance::dsl::balance)
	    .values((schema::balance::dsl::user_id.eq(user_id), schema::balance::dsl::points.eq(points)))
	    .execute(&mut db)
	    .expect("Error saving new post");
    }
    Ok(())
}
