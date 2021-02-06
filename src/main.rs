use anyhow::Result;
use clap::Clap;
use twitter_api::Client;

mod config;
mod twitter_api;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "earlgray <@earlgray329>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Login,
    Logout,
    Tweet(Tweet),
}

#[derive(Clap)]
struct Login;
#[derive(Clap)]
struct Tweet {
    text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Login => {
            config::create_config_file().await?;
        }
        SubCommand::Tweet(tweet) => {
            let client = Client::from_config().await?;
            let _res = client.tweet(&tweet.text).await?;
        }
        SubCommand::Logout => {
            todo!();
        }
    }

    Ok(())
}
