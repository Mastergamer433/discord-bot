mod points;

use crate::structs::{
    FrameworkContext,
    Result,
};

use poise::serenity_prelude as serenity;
use points::*;
use self::serenity::{
    Context,
    Message,
};

pub async fn message(_ctx: &Context, _framework: FrameworkContext<'_>, new_message: &Message) -> Result<()> {
    tokio::try_join!(
	points(_ctx, _framework, new_message)
    )?;

    Ok(())
}
