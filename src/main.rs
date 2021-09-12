use clap::{load_yaml, App};

use spammer_bot::Args;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let argv = App::from(yaml).get_matches();

    let discord_token = argv.value_of("discord-token").unwrap().to_owned();
    let args = Args::new(discord_token);
    spammer_bot::run(args).await
}
