# RTWI (Real Twitter)
## What is RTWI?
 On twitter, you don't need to find other person's tweets because `twitter` means `a person who tweet`. On RTWI, you can only tweet, so you can experience `Real Twitter`. 

### High Performance
Here is a table whith includes times to tweet 'hello everyone' for each client.
| Client | time(sec) |
| --- | --- |
| Twitter Web App | 8.63 |
| **rtwi** | **7.34** |

## Installation
1. Install from github  
```console
$ git clone https://github.com/earlgray283/rtwi.git
$ cd rtwi
$ cargo build --release
$ cp ./target/release/rtwi {path/to/dir}
```
or
```console
$ cargo install rtwi
```

## Usage
### 1. Login  
```console
$ rtwi login
...
```
also you can login by creating `$HOME/.config/rtwi/Config.toml` 
```console
$ mkdir -p $HOME/.config/rtwi/
$ echo '
api_key = "api_keyyyy!!!!"
api_secret_key = "api_secret_keyyyy!!!!"
access_token = "access_tokennnn!!!!"
access_token_secret = "amazing_mightyyyy!!!!"' > $HOME/.config/rtwi/Config.toml
```

#### Attension
`Config.toml` is located on `$HOME/.config/rtwi`

### 2. tweet  
```console
$ rtwi tweet 'hello from rtwi. I use †Real Twitter†.'
status: tweeted
```

### 3. That's all :D

## Configuration
If you hope, you can escape from `Real Twitter` and can watch `Timeline` on rtwi.  
```toml
[config]
real_twitter = false
```