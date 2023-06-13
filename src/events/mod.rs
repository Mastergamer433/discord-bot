mod message;
mod interaction_create;
mod presence_update;

use crate::structs::{
    FrameworkContext,
    Result,
    Data,
};
use message::*;
use interaction_create::*;
use presence_update::*;
use poise::event::Event;
use poise::serenity_prelude::Context;

pub async fn handler(_ctx: &Context, event: &Event<'_>, _framework: FrameworkContext<'_>, _data: &Data) -> Result<()> {
    println!("Event: {}", event.name());
    match event {
        Event::Message { new_message } => message(_ctx, _framework, new_message).await,
        Event::InteractionCreate { interaction } => interaction_create(_ctx, _framework, interaction).await,
        Event::PresenceUpdate { new_data } => presence_update(_ctx, _framework, new_data).await,

        _ => Ok(()),
    }
}
