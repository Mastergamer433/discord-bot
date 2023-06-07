mod get_options;
mod check_permissions;
mod transfer;

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




