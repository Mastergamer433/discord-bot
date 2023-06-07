use poise::serenity_prelude::User;

use crate::{
    structs::{
	CommandResult,
	Context
    },
    models::Balance,
    helpers::transfer,
    helpers::get_option,
};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use poise::serenity_prelude as serenity;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn gift(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "User"] user: User,
    #[description = "Amount"] amount: f32,
) -> CommandResult {
    let receiver_user_id = user.id.as_u64().to_string();
    let sender_user_id = &ctx.author().id.as_u64().to_string();
    let transfer = transfer(ctx.data().db_pool.get().expect("Could not get database"), &sender_user_id, &receiver_user_id, amount).await;
    println!("{}",
		get_option(
		    ctx.data()
			.db_pool
			.get()
			.expect("Could not get database"),
		    ctx.guild()
			.unwrap()
			.id,
		    user.id,
		    "contact.email.address").await.expect("Could not get option"));
    let message = Message::builder()
	.from("discord-bot <discord-bot@kimane.se>".parse().unwrap())
	.reply_to("Elis Odenhage <mg433@kimane.se>".parse().unwrap())
	.to(
	    format!(
		"{} <{}>",
		user.name,
		get_option(
		    ctx.data()
			.db_pool
			.get()
			.expect("Could not get database"),
		    ctx.guild()
			.unwrap()
			.id,
		    user.id,
		    "contact.email.address").await.expect("Could not get option")).parse()
		.unwrap())
	.date_now()
	.subject("Happy new year")
	.header(ContentType::TEXT_HTML)
        .body(String::from(format!("<h1>{} has gifted {} point to you</h1>", ctx.author().name, amount)))
	.unwrap();

    let creds = Credentials::new(ctx.data().config.email.user.to_owned(), ctx.data().config.email.pass.to_owned());

    let mailer = SmtpTransport::starttls_relay(&ctx.data().config.email.server)
	.unwrap()
	.credentials(creds)
	.port(ctx.data().config.email.port)
	.build();

    // Send the email
    match mailer.send(&message) {
	Ok(_) => println!("Email sent successfully!"),
	Err(e) => panic!("Could not send email: {e:?}"),
    }

    Ok(())
}
