
pub async fn get_option(
    mut db: PooledConnection<ConnectionManager<MysqlConnection>>,
    g_id: GuildId,
    u_id: UserId,
    option: &str,
) -> Result<String, Error> {
    use diesel::select;
    use diesel::dsl::exists;
    use schema::options::dsl::*;

    let opt = select(
	exists(
	    options
		.filter(option.eq(option))
		.filter(user_id.eq(u_id.as_u64().to_string()))
		.filter(guild_id.eq(g_id.as_u64().to_string()))))
	.get_result(&mut db)
	.expect("Error checking if permission exist");
    if opt {
	let opt: Options = options
	    .filter(option.eq(option))
	    .filter(user_id.eq(u_id.as_u64().to_string()))
	    .filter(guild_id.eq(g_id.as_u64().to_string()))
	    .get_result(&mut db).expect("Error getting balance");
	Ok(opt.value)
    } else {
	return Err(anyhow!("Cant get option..."));
    }
}
