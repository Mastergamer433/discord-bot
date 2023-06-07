pub mod get_options;
pub mod check_permissions;
pub mod transfer;

use crate::{
    schema,
    models::{
	Balance,
	Permissions,
	Options,
    },
    structs::{
	TransferError,
	Context
    },
};

use anyhow::Error;
use anyhow::anyhow;
use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    r2d2::Pool,
};
use poise::serenity_prelude::{self as serenity, GuildId, UserId};




