mod message;
mod interaction_create;

use crate::structs::{
    FrameworkContext,
    Result,
    Data,
};
use message::*;
use interaction_create::*;
use poise::event::Event;
use poise::serenity_prelude::Context;


pub async fn handler(_ctx: &Context, event: &Event<'_>, _framework: FrameworkContext<'_>, _data: &Data) -> Result<()> {
    println!("Event: {}", event.name());
    match event {
        Event::Message { new_message } => message(_ctx, _framework, new_message).await,
        Event::InteractionCreate { interaction } => interaction_create(_ctx, _framework, interaction).await,

        _ => Ok(()),
    }
}
