use std::time::Duration;

use crate::parse;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tokio::time;

async fn spam_for(ctx: Context, msg: &Message, username: String, interval: f32, repeat: usize) {
    if interval < 0.5 {
        super::reply_message(&ctx, msg, "cannot use interval shorter than 0.5s").await;
        return;
    }

    if repeat as f32 / interval > 20.0 {
        super::reply_message(&ctx, msg, "sorry, but you can't spam this much").await;
        return;
    }

    println!(
        "{} spammed {} messages every {} seconds",
        msg.author.name, repeat, interval
    );

    let duration = Duration::from_secs_f32(interval);
    let mut timer = time::interval(duration);
    for _ in 0..repeat {
        timer.tick().await;
        super::send_message(&ctx, msg, &username).await;
    }
}

async fn spam_once(ctx: Context, msg: &Message, username: String, repeat: usize) {
    const MAX_REPEAT: usize = 80;
    if repeat > MAX_REPEAT {
        super::reply_message(
            &ctx,
            msg,
            format!("cannot spam more than {} times at once", MAX_REPEAT),
        )
        .await;
        return;
    }

    let message: String = std::iter::once(username).cycle().take(repeat).collect();
    if message.len() > 1800 {
        super::reply_message(&ctx, msg, "message is too long").await;
    } else {
        println!("{} spammed {} messages at once", msg.author.name, repeat);
        super::send_message(&ctx, msg, message).await;
    }
}

pub async fn spam(ctx: Context, msg: &Message, args: parse::SpamArgs) {
    use parse::SpamArgsKind::*;
    match args.kind {
        SpamFor(fargs) => spam_for(ctx, msg, args.username, fargs.interval, fargs.repeat).await,
        SpamOnce(oargs) => spam_once(ctx, msg, args.username, oargs.repeat).await,
    }
}
