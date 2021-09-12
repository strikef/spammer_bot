use std::time::Duration;

use crate::parse;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use tokio::time;

async fn spam_for(
    ctx: Context,
    channel_id: ChannelId,
    username: String,
    interval: f32,
    repeat: usize,
) {
    if interval < 0.5 {
        super::send_message(&ctx, channel_id, "cannot use interval shorter than 0.5s").await;
        return;
    }

    if repeat as f32 / interval > 20.0 {
        super::send_message(&ctx, channel_id, "sorry, but this is too much").await;
        return;
    }

    let duration = Duration::from_secs_f32(interval);
    let mut timer = time::interval(duration);
    for _ in 0..repeat {
        timer.tick().await;
        super::send_message(&ctx, channel_id, &username).await;
    }
}

async fn spam_once(ctx: Context, channel_id: ChannelId, username: String, repeat: usize) {
    const MAX_REPEAT: usize = 80;
    if repeat > MAX_REPEAT {
        super::send_message(
            &ctx,
            channel_id,
            format!("cannot spam more than {} times at once", MAX_REPEAT),
        )
        .await;
        return;
    }

    let message: String = std::iter::once(username).cycle().take(repeat).collect();
    super::send_message(&ctx, channel_id, message).await;
}

pub async fn spam(ctx: Context, channel_id: ChannelId, args: parse::SpamArgs) {
    use parse::SpamArgsKind::*;
    match args.kind {
        SpamFor(fargs) => {
            spam_for(ctx, channel_id, args.username, fargs.interval, fargs.repeat).await
        }
        SpamOnce(oargs) => spam_once(ctx, channel_id, args.username, oargs.repeat).await,
    }
}
