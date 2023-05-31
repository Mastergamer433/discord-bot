use poise::serenity_prelude::UserId;
use serde::Deserialize;
use serenity::model::prelude::GuildId;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot: Bot,
    pub guild_id: GuildId,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    pub token: String,
    pub userId: UserId,
}
