use anyhow::{anyhow, Result};
use serde_derive::{Deserialize, Serialize};
use std::{
    env,
    fs::{create_dir, remove_file, File},
    io::{stdout, Read, Write},
    path::Path,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub twitter_api_info: TwitterApiInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TwitterApiInfo {
    pub api_key: String,
    pub api_secret_key: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut file = open_config_file()?;

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let config = match toml::from_str::<Config>(&buf) {
            Ok(config) => config,
            Err(e) => {
                return Err(anyhow!(format!(
                    "The contents of Config.toml have changed since v0.2.0.
Sorry, please run 'rtwi login' again.\n{:?}",
                    &e
                )))
            }
        };

        Ok(config)
    }
}

pub fn create_config_dir() -> Result<()> {
    let config_dir_path = format!("{}/.config/rtwi", env::var("HOME").unwrap());
    if !Path::new(&config_dir_path).exists() {
        create_dir(config_dir_path)?;
    }

    Ok(())
}

pub fn create_config_file() -> Result<()> {
    let config_file_path = format!("{}/.config/rtwi/Config.toml", env::var("HOME").unwrap());

    let config = input_config();
    let toml = toml::to_string(&config)?;

    let mut file = File::create(&config_file_path)?;
    if let Err(e) = file.write_all(toml.as_bytes()) {
        remove_file(&config_file_path)?;
        return Err(anyhow!(e));
    }

    Ok(())
}

pub fn open_config_file() -> Result<File> {
    let config_file_path = format!("{}/.config/rtwi/Config.toml", env::var("HOME").unwrap());
    if !Path::new(&config_file_path).exists() {
        return Err(anyhow!(
            r#"
    You don't seem to login yet :(
    Please run '$ rtwi login'"#
        ));
    }

    let file = File::open(&config_file_path)?;

    Ok(file)
}

pub fn input_config() -> Config {
    print!(
        r#"
Welcome to RTWI!
To use RTWI, you must generate "$HOME/.config/rtwi/Config.toml" following the steps below.
Steps: 
  1. Access to "https://apps.twitter.com/app/new" and create an app.
  2. Please input "api_key", "api_secret_key", "access_token", "access_token_secret" in order.
"#
    );
    stdout().flush().unwrap();

    let stdin = std::io::stdin();

    let mut api_key;
    let mut api_secret_key;
    let mut access_token;
    let mut access_token_secret;

    loop {
        api_key = read_string(&stdin, "    api_key = ");
        api_secret_key = read_string(&stdin, "    api_secret_key = ");
        access_token = read_string(&stdin, "    access_token = ");
        access_token_secret = read_string(&stdin, "    access_token_secret = ");

        let prmt = format!(
            r#"
================== Confirm ==================
    api_key = {}
    api_secret_key = {}
    access_token = {}
    access_token_secret = {}

If you are OK, then please type y, else type n > "#,
            api_key, api_secret_key, access_token, access_token_secret
        );
        let yn = read_string(&stdin, &prmt);
        stdout().flush().unwrap();
        if yn.trim_end() == "y" {
            println!();
            break;
        }
    }

    println!("3. Please input your twitter screen_name. (example: screen_name = @earlgray329)");

    let mut name;
    loop {
        name = read_string(&stdin, "    screen_name = ");
        if name.strip_prefix('@').is_none() {
            println!("screen_name must have a prefix of @. (example: screen_name = @earlgray329)");
            continue;
        }

        break;
    }

    Config {
        name,
        twitter_api_info: TwitterApiInfo {
            api_key,
            api_secret_key,
            access_token,
            access_token_secret,
        },
    }
}

fn read_string(stdin: &std::io::Stdin, prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}
