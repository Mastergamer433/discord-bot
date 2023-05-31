#![warn(clippy::str_to_string)]

mod commands;
mod config;
mod error;

use config::Config;
use discord_bot::models::NewBalance;
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
use poise::event::Event;
use tracing::info;
use std::{fs::File, io::BufReader, env, collections::HashMap, env::var, sync::Mutex, time::Duration};

pub struct Data {
    config: Config,
    dbPool: DbPool,
    votes: Mutex<HashMap<String, u32>>,
    points: u32
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const CONFIG_PATH: &str = "./config.json";

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

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
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

fn init_config() -> Result<Config, ConfigError> {
    let file = File::open(CONFIG_PATH)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = init_config()?;
    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::vote(), commands::getvotes()],
        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
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
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
		match event {
		    Event::Message { new_message } => {
			println!("Author: {}, Content: {}", new_message.author.name, new_message.content);
                        let mut db = get_db_pool().get().expect("Couldn't get db connection from pool");

			let points: &f32 = new_message.content.chars().count() as i32 * 0.06;
			let user_id: &str = new_message.author.id;
			let new_balance = NewBalance { user_id, points };

			diesel::insert_into(schema::balance::table)
			    .values(&new_balance)
			    .execute(&mut db)
			    .expect("Error saving new post");
			()
		    },

		    _ => (),
		};
                Ok(())
            })
        },
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
		    dbPool: get_db_pool(),
                    votes: Mutex::new(HashMap::new()),
		    points: 0
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
    Ok(())
}
