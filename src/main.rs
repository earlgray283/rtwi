use anyhow::Result;
use clap::Clap;
use config::Config;
use twitter_api::Client;

mod config;
mod twitter_api;
mod sub_command;

#[derive(Clap)]
#[clap(version = "0.2.0", author = "earlgray <@earlgray329>")]
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

            let res = client.tweet(&tweet.text).await;

            println!(
                "status: {}",
                if res.is_ok() {
                    "tweeted".to_string()
                } else {
                    format!("error\n{:?}", res.map_err(|e| e))
                }
            )
        }
        SubCommand::Logout => {
            todo!();
        }
        SubCommand::Status => {
            let config = Config::new()?;
            let client = Client::new(&config);

            let user_info = client.get_profile(Some(&config.name), None).await?;
            sub_command::show_profile(&user_info);
        }
    }

    Ok(())
}
