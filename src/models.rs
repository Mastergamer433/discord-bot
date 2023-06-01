use diesel::prelude::*;
use crate::schema::balance;

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
