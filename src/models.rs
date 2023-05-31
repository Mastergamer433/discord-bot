use diesel::prelude::*;
use poise::serenity_prelude::UserId;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::balance)]
pub struct Balance {
    pub id: i32,
    pub user_id: UserId,
    pub points: i32
}
