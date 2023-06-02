pub use anyhow::{Error, Result};
use crate::config::Config;
use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::ConnectionManager,
    r2d2::Pool,
};
use std::{fs::File, io::BufReader, env, collections::HashMap, env::var, sync::Mutex, time::Duration};

pub type Command = poise::Command<Data, CommandError>;

pub struct Data {
    pub config: Config,
    pub db_pool: DbPool,
    pub votes: Mutex<HashMap<String, u32>>,
    pub points: u32
}

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type CommandResult<E=Error> = Result<(), E>;
pub type CommandError = Error;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, CommandError>;
