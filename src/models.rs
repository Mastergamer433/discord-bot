use diesel::prelude::*;
use crate::schema::balance;
use crate::schema::permissions;
use crate::schema::options;

#[derive(Queryable, Selectable)]
#[diesel(table_name = balance)]
pub struct Balance {
    pub id: i32,
    pub user_id: String,
    pub points: f32,
}

#[derive(Insertable)]
#[diesel(table_name = balance)]
pub struct NewBalance<'a> {
    pub user_id: &'a str,
    pub points: &'a f32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = permissions)]
pub struct Permissions {
    pub id: i32,
    pub user_id: String,
    pub guild_id: String,
    pub permission_string: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = options)]
pub struct Options {
    pub id: i32,
    pub user_id: String,
    pub guild_id: String,
    pub option: String,
    pub value: String,
}
