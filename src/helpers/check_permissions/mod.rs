use crate::{
    schema,
    models::{
	Permissions,
    },
    structs::{
	Context
    },
};

use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    r2d2::Pool,
};

use anyhow::Error;

use poise::serenity_prelude::{self as serenity, GuildId, UserId};
pub async fn check_permission(
    mut db: PooledConnection<ConnectionManager<MysqlConnection>>,
    g_id: GuildId,
    u_id: UserId,
    permission: &str,
) -> Result<bool, Error> {
    use diesel::select;
    use diesel::dsl::exists;
    use schema::permissions::dsl::*;

    let perm = select(
	exists(
	    permissions
		.filter(user_id.eq(u_id.as_u64().to_string()))
		.filter(guild_id.eq(g_id.as_u64().to_string()))))
	.get_result(&mut db)
	.expect("Error checking if permission exist");
    if perm {
	let perm: Permissions = permissions
	    .filter(user_id.eq(u_id.as_u64().to_string()))
	    .filter(guild_id.eq(g_id.as_u64().to_string()))
	    .get_result(&mut db).expect("Error getting balance");
	let perms: Vec<&str> = perm.permission_string.split(",").collect();

	if perms.contains(&permission) {
	    return Ok(true);
	} else {
	    return Ok(false);
	}
    } else {
	return Ok(false);
    }

    Ok(false)
}
