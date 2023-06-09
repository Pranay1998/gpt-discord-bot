use std::env;

const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const OPENAI_TOKEN: &str = "OPENAI_TOKEN";

#[tokio::main]
async fn main() -> Result<(), lib::ServerError> {
    println!("Server starting with pid {}...", std::process::id());
    let discord_token = env::var(DISCORD_TOKEN)?;
    let openai_token = env::var(OPENAI_TOKEN)?;

    lib::start_server(discord_token, openai_token).await?;
    Ok(())
}