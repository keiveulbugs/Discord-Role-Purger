
mod commands;
use poise::serenity_prelude::{self as serenity};
use text_io::read;
type Error = Box<dyn std::error::Error + Send + Sync>;

#[macro_use]
//.env variables
extern crate dotenv_codegen;

//Constants
// Your Bot token
const DISCORD_TOKEN: &str = dotenv!("DISCORD_TOKEN");
// If you want to have commands specific to only a specific guild, set this as your guild_id.
//const PRIVATEGUILDID: serenity::GuildId = serenity::GuildId(1014660478351454299);

async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<(), Error>,
) -> Result<(), Error> {
    // To announce that the bot is online.
    println!("{} is connected!", ready.user.name);

    // This registers commands for the bot, guild commands are instantly active on specified servers
    //
    // The commands you specify here only work in your own guild!
    // This is useful if you want to control your bot from within your personal server,
    // but dont want other servers to have access to it.
    // For example sending an announcement to all servers it is located in.
    let builder = poise::builtins::create_application_commands(&framework.options().commands);
    // let commands =
    //     serenity::GuildId::set_application_commands(&PRIVATEGUILDID, &ctx.http, |commands| {
    //         *commands = builder.clone();

    //         commands
    //     })
    //     .await;
    // // This line runs on start-up to tell you which commands succesfully booted.
    // if commands.is_err() {
    //     println!("Failed to deploy guild commands\n{:#?}", commands);
    // } else {
    //     println!("Succesfully deployed guild commands!");
    // };

    // Below we register Global commands, global commands can take some time to update on all servers the bot is active in
    //
    // Global commands are availabe in every server, including DM's.
    // We call the commands folder, the ping file and then the register function.
    let global_command1 =
        serenity::Command::set_global_application_commands(&ctx.http, |commands| {
            *commands = builder;
            commands
        })
        .await;
    if global_command1.is_err() {
        println!("Failed to deploy global commands\n{:#?}", global_command1);
    } else {
        println!("Succesfully deployed global commands!");
    };

    Ok(())
}

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    // Build our client.
    let client = poise::Framework::builder()
        .token(DISCORD_TOKEN)
        .intents(serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::GUILD_MEMBERS)
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::purge::purge(),
            ],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
        .build()
        .await
        .expect("Error creating client");

    println!("Checking for secret key");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}\nTry entering a working bot-secret:", why);
        let token :String = read!();
        let client = poise::Framework::builder()
        .token(token)
        .intents(serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::GUILD_MEMBERS)
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::purge::purge(),
            ],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
        .build()
        .await
        .expect("Error creating client");

        if let Err(why2) = client.start().await {
            println!("Error again, shutting down. \n {}", why2);
        }
        
    }
}

    