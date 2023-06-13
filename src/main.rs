#![warn(clippy::str_to_string)]

mod commands;
mod events;
mod config;
mod error;
mod schema;
mod models;
mod structs;
mod helpers;

use config::Config;
use models::{NewBalance, Balance};
use structs::{
    DbPool,
    Context,
    Data,
};
use error::ConfigError;
use dotenvy::dotenv;
use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::ConnectionManager,
    r2d2::Pool,
};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::async_trait;
use tracing::{info, error};
use std::{fs::File, io::BufReader, env, collections::HashMap, env::var, sync::Mutex, time::Duration};

const CONFIG_PATH: &str = "./config.json";

pub fn get_db_pool() -> DbPool {

    let database_url =
        env::var("DATABASE_URL").expect("Missing the DATABASE_URL environment variable");

    info!("Connecting to database {}...", database_url);
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    let mut db = pool.get().expect("Couldn't get db connection from pool");
    pool
}

fn init_config() -> Result<Config, ConfigError> {
    let file = File::open(CONFIG_PATH)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = init_config()?;
    let options = poise::FrameworkOptions {
        commands: commands::commands(),
        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(async move {
	    match error {
		poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
		poise::FrameworkError::Command { error, ctx } => {
		    println!("Error in command `{}`: {:?}", ctx.command().name, error,);
		}
		error => {
		    if let Err(e) = poise::builtins::on_error(error).await {
			println!("Error while handling error: {}", e)
		    }
		}
	    }
	}),
        /// This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        /// This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == ctx.data().config.bot.userId{
		    println!("Bot");
                    return Ok(false);
                }
		println!("Not bot");
                Ok(true)
            })
        }),
        /// Enforce command checks even for owners (enforced by default)
        /// Set to true to bypass checks, which is useful for testing
        event_handler: |_ctx, event, _framework, _data| Box::pin(events::handler(_ctx, event, _framework, _data)),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(&config.bot.token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
		    config: config,
		    db_pool: get_db_pool(),
                    votes: Mutex::new(HashMap::new()),
		    points: 0
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_PRESENCES,
        )
        .run()
        .await
        .unwrap();
    Ok(())
}
