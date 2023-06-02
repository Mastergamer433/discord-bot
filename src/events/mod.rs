mod message;

use crate::structs::{
    FrameworkContext,
    Result,
    Data,
};
use message::*;
use poise::event::Event;
use poise::serenity_prelude::Context;


pub async fn handler(_ctx: &Context, event: &Event<'_>, _framework: FrameworkContext<'_>, _data: &Data) -> Result<()> {

    match event {
        Event::Message { new_message } => message(_ctx, _framework, new_message).await,

        _ => Ok(()),
    }
}
