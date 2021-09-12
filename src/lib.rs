use std::collections::HashSet;
use std::sync::Mutex;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, UserId},
    },
    prelude::*,
};

mod parse;
mod spam;

async fn send_message(ctx: &Context, channel_id: ChannelId, content: impl std::fmt::Display) {
    if let Err(why) = channel_id.say(&ctx.http, content).await {
        println!("Error sending message: {:?}", why);
    };
}

struct Handler {
    blocked_users: Mutex<HashSet<UserId>>,
}

impl Handler {
    fn new() -> Self {
        Self {
            blocked_users: Mutex::new(HashSet::new()),
        }
    }

    fn check_queue(&self, user_id: UserId) -> bool {
        self.blocked_users.lock().unwrap().insert(user_id)
    }

    fn dequeue(&self, user_id: UserId) -> bool {
        self.blocked_users.lock().unwrap().remove(&user_id)
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("'sp") {
            if self.check_queue(msg.author.id) {
                match parse::parse_command(msg.content.clone()) {
                    Ok(args) => {
                        spam::spam(ctx, msg.channel_id, args).await;
                        self.dequeue(msg.author.id);
                    }
                    Err(e) => send_message(&ctx, msg.channel_id, e).await,
                }
            } else {
                println!(
                    "spam request from {} is blocked until previous request is completed",
                    msg.author.name
                );
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub struct Args {
    discord_token: String,
}

impl Args {
    pub fn new(discord_token: String) -> Self {
        Self { discord_token }
    }
}

pub async fn run(args: Args) {
    let handler = Handler::new();
    let mut client = Client::builder(&args.discord_token)
        .event_handler(handler)
        .await
        .expect("Failed to create discord bot client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
