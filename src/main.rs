use anyhow::Result;
use clap::Clap;
use config::Config;
use twitter_api::Client;

mod config;
mod twitter_api;

#[derive(Clap)]
#[clap(version = "0.1.12", author = "earlgray <@earlgray329>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Login,
    Logout,
    Tweet(Tweet),
    Status,
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

    config::create_config_dir()?;

    match opts.subcmd {
        SubCommand::Login => config::create_config_file()?,
        SubCommand::Tweet(tweet) => {
            let config = Config::new()?;
            let client = Client::new(&config);
            let _res = client.tweet(&tweet.text).await?;
        }
        SubCommand::Logout => {
            todo!();
        }
        SubCommand::Status => {
            let config = Config::new()?;
            let client = Client::new(&config);
            let res = client.show_profile(Some(&config.name), None).await?;
            println!("{:?}", &res);
        }
    }

    Ok(())
}
