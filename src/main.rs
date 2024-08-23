use poise::serenity_prelude as serenity;
use std::process::Command;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, global_cooldown = 1800)]
async fn reboot(ctx: Context<'_>) -> Result<(), Error> {
    {
        let allowed_users: Vec<u64> = vec![
            209482079535104001, // Wil
            217784600922030081, // Sac
            217099464362557440, // Dup
            290346790241173504, // Say
            343836642425176064, // Me
        ];

        if allowed_users.contains(&ctx.author().id.into()) {
            let output = Command::new("systemctl")
                .args(["restart", "minecraft-server.service"])
                .output()?;

            ctx.say(format!(
                "Stop crashing stuff \nLog: {}",
                String::from_utf8_lossy(&output.stdout)
            ))
            .await?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![reboot()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::Client::builder(token, serenity::GatewayIntents::non_privileged())
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
