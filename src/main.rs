mod commands;

extern crate config;

use std::env;
use std::collections::HashMap;

use serde_json::{Number, Value};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::{fs::File, io::BufReader};

struct Handler;
struct ConfigContainer;
impl TypeMapKey for ConfigContainer {
  type Value = Value;
}

const CONFIG_PATH: &str = "../config.json";

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                "attachmentinput" => commands::attachmentinput::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::welcome::register(command))
                .create_application_command(|command| commands::numberinput::register(command))
                .create_application_command(|command| commands::attachmentinput::register(command))
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::wonderful_command::register(command)
        })
        .await;

        println!("I created the following global slash command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    let config: Value = {
        let file_result = File::open(CONFIG_PATH);
	let file = match file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the config file: {:?}", error),
	};
        let reader = BufReader::new(file);
        let json_result = serde_json::from_reader(reader);
	let json = match json_result {
            Ok(json) => json,
            Err(error) => panic!("Problem deserializing the config file: {:?}", error),
	};
	json
    };
    // Configure the client with your Discord bot token in the environment.
    let token = &config["bot"]["token"];

    println!("{}", token);
    // Build our client.
  //   let mut client = Client::builder(&token, GatewayIntents::empty())
  // 	.event_handler(Handler)
  // 	.await
  // 	.expect("Err creating client");
  // {
  //   let mut data = client.data.write().await;

  //   data.insert::<ConfigContainer>(config);
  // }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    // if let Err(why) = client.start().await {
    //     println!("Client error: {:?}", why);
    // }
}