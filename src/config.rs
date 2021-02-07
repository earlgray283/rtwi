use std::{env, fs::create_dir, io::{Write, stdout}, path::Path};
use anyhow::Result;
use tokio::{fs::File, io::AsyncWriteExt};

pub fn create_config_dir() -> Result<()> {
    let config_dir_path = format!("{}/.config/rtwi", env::var("HOME").unwrap());
    if !Path::new(&config_dir_path).exists() {
        create_dir(config_dir_path)?;
    }

    Ok(())
}

pub async fn create_config_file() -> Result<()> {
    let config_file_path = format!("{}/.config/rtwi/Config.toml", env::var("HOME").unwrap());
    
    let mut file = File::create(&config_file_path).await?;
    if let Err(e) = input_config(&mut file).await {
        std::fs::remove_file(&config_file_path).unwrap();
        return Err(e);
    }

    Ok(())
}

pub async fn open_config_file() -> Result<File> {
    let config_file_path = format!("{}/.config/rtwi/Config.toml", env::var("HOME").unwrap());
    if !Path::new(&config_file_path).exists() {
        return Err(anyhow::Error::msg("It seems that you didn't create config file :("));
    }

    let file = File::open(&config_file_path).await?;

    Ok(file)
}

pub async fn input_config(file: &mut File) -> Result<()> {
    print!(
        r#"
Welcome to RTWI!
To use RTWI, you must generate "$HOME/.config/rtwi/Config.toml" following the steps below.
Steps: 
  1. Access to "https://apps.twitter.com/app/new" and create an app.
  2. Please input "api_key", "api_secret_key", "access_token", "access_token_secret" in order.
"#
    );

    let stdin = std::io::stdin();

    loop {
        print!("api_key = ");
        stdout().flush().unwrap();
        let mut api_key = String::new();
        stdin.read_line(&mut api_key).unwrap();
        api_key = api_key.trim_end().to_string();

        print!("api_secret_key = ");
        stdout().flush().unwrap();
        let mut api_secret_key = String::new();
        stdin.read_line(&mut api_secret_key).unwrap();
        api_secret_key = api_secret_key.trim_end().to_string();

        print!("access_token = ");
        stdout().flush().unwrap();
        let mut access_token = String::new();
        stdin.read_line(&mut access_token).unwrap();
        access_token = access_token.trim_end().to_string();

        print!("access_token_secret = ");
        stdout().flush().unwrap();
        let mut access_token_secret = String::new();
        stdin.read_line(&mut access_token_secret).unwrap();
        access_token_secret = access_token_secret.trim_end().to_string();

        print!(
            r#"
        === Confirm ===
        api_key = {}
        api_secret_key = {}
        access_token = {}
        access_token_secret = {}
        "#,
            api_key, api_secret_key, access_token, access_token_secret
        );

        print!("Please input y or n > ");
        stdout().flush().unwrap();
        let mut yn = String::new();
        stdin.read_line(&mut yn).unwrap();

        if yn.trim_end() == "y".to_string() {
            let tmp = super::twitter_api::Client {
                api_key,
                api_secret_key,
                access_token,
                access_token_secret,
            };

            let toml = toml::to_string(&tmp)?;
            file.write_all(toml.as_bytes()).await?;
            break;
        }
    }

    Ok(())
}
