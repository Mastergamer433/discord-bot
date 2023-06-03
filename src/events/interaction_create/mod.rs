use crate::structs::{
    FrameworkContext,
    Result,
};

use poise::serenity_prelude as serenity;
use self::serenity::{
    Context,
    Interaction,
};

pub async fn interaction_create(
    _ctx: &Context,
    _framework: FrameworkContext<'_>,
    interaction: &Interaction
) -> Result<()> {
    Ok(())
}
