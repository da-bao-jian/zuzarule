mod bot;
mod consts;
mod errors;
mod handler;
mod keyboards;
mod messages;
mod storage;
mod utils;
use tracing_subscriber::EnvFilter;

pub use self::errors::TgError;

#[tokio::main]
pub async fn main() -> Result<(), TgError> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    log::info!("Starting buttons bot...");

    let bot = bot::TgBot::new();
    let _ = bot.init().await;

    Ok(())
}
