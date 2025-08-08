mod commands;

use commands::commands;
use dotenvy::dotenv;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut commands = commands();
    let matches = commands.get_matches_mut();

    match matches.subcommand() {
        Some(("create-app", matches)) => commands::create_app::run(matches).await,
        Some(("password-hash", matches)) => commands::password_hash::run(matches),
        Some(("seed", matches)) => commands::seed::run(matches).await,
        Some(("setup", matches)) => commands::setup::run(matches).await,
        Some(("migrate", matches)) => commands::migrate::run(matches).await,
        e => unreachable!("{:?}", e),
    }
}
