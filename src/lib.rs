pub mod models;
pub mod schema;

use self::models::{NewBalance, Balance};
use diesel::*;
use poise::serenity_prelude::UserId;

pub fn create_post(conn: &mut MysqlConnection, user_id: UserId, points: u32) -> Balance {
    use crate::schema::balance;

    let new_balance = NewBalance { user_id, points };

    diesel::insert_into(balance::table)
        .values(&new_balance)
        .returning(Balance::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
