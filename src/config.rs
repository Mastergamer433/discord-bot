use poise::serenity_prelude::UserId;
use serde::Deserialize;
use serenity::model::prelude::GuildId;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot: Bot,
    pub guild_id: GuildId,
    pub email: Email,
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub port: u16,
    pub server: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    pub token: String,
    pub userId: UserId,
}
