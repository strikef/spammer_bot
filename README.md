# SpammerBot

A discord bot to spam some messages ;)

## How to call
```
# repeats message every <interval> seconds <repeat> times
'sp <message/mention> <interval(sec)> <repeat>
# copy-pastes message <repeat> times in a single message
'spo <message/mention> <repeat>
```

### Limitations
In order to prevent *excessive* spamming, there are some limits on this bot

#### Limits on all commands
- One cannot issue new command before the previous one finishes running

#### Limits on **'sp**
- The minimum interval is 0.5 seconds
- The repeat divided by interval should not be greater than 20  
(for example, you can't send more than 10 messages using 0.5s interval, as 10 / 0.5 == 20)

#### Limits on **'spo**
- You can't repeat more than 80 messages at once

## How to run
### Prerequisites
This bot is written in Rust, and requires cargo toolchain to build and execute  
This bot has been developed using Rust 1.54.
Note that [serenity](https://github.com/serenity-rs/serenity) requires Rust 1.48+.

### Running the bot
```bash
cargo run --release --discord-token=<your discord token here>
```

## Open-source crates used
[serenity](https://github.com/serenity-rs/serenity) under ISC License,
[clap](https://github.com/clap-rs/clap) and
[tokio](https://github.com/tokio-rs/tokio) under MIT License
