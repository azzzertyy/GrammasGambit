use poise::serenity_prelude as serenity;
use rand::Rng;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Cmon, why wait? Gramma will decide your fate.
#[poise::command(slash_command, prefix_command)]
async fn gambit(ctx: Context<'_>) -> Result<(), Error> {
    let x: i32 = rand::thread_rng().gen_range(1..1003);
    let mut result = "not";
    match x {
        1..=2 => result = "https://cdn.discordapp.com/attachments/1026290034980167720/1026290159634886667/gramma_gamU.gif",
        3..=502 => result = "https://cdn.discordapp.com/attachments/1026290034980167720/1026290160410841098/gramma_gam1.gif",
        _ => result = "https://cdn.discordapp.com/attachments/1026290034980167720/1026290160012369920/gramma_gam2.gif",
    };
    ctx.say(result).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![gambit(),register(),],
            ..Default::default()
        })
        .token("")
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
