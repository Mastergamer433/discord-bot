use crate::{
    schema,
    models::{
	Balance,
    },
    structs::{
	TransferError,
	Context
    },
};

use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    r2d2::Pool,
};

pub async fn transfer(
    mut db: PooledConnection<ConnectionManager<MysqlConnection>>,
    sender_user_id: &String,
    receiver_user_id: &String,
    amount: f32
) -> Result<(), TransferError> {
    use diesel::select;
    use diesel::dsl::exists;
    use schema::balance::dsl::*;

    let bal = select(exists(balance.filter(user_id.eq(sender_user_id))))
	.get_result(&mut db)
	.expect("Error checking if balance exist");
    if bal {
	let bal: Balance = balance.filter(user_id.eq(sender_user_id)).get_result(&mut db).expect("Error getting balance");
	if bal.points < amount {
	    return Err(TransferError::NotEnoughPoints);
	} else {
	    let bal = select(exists(balance.filter(user_id.eq(receiver_user_id))))
		.get_result(&mut db)
		.expect("Error checking if balance exist");
	    if bal {
		diesel::update(balance.filter(user_id.eq(receiver_user_id)))
		    .set(points.eq(points+amount))
		    .execute(&mut db)
		    .unwrap();
	    } else {
		diesel::insert_into(balance)
		    .values((user_id.eq(receiver_user_id), points.eq(amount)))
		    .execute(&mut db)
		    .expect("Error saving new post");
	    }
	    diesel::update(balance.filter(user_id.eq(sender_user_id)))
		.set(points.eq(points-amount))
		.execute(&mut db)
		.unwrap();
	}
    } else {
	return Err(TransferError::NoBalance);
    }

    Ok(())
}
