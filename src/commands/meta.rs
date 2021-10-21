use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn purge(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Attempting to purge...")
        .await?;
    if let Some(guild) = msg.guild_id {
        msg.channel_id.say(&ctx.http, "Acquired guild...").await?;
        if let Some(name) = args.current() {
            msg.channel_id.say(&ctx.http, "Parsed arguments...").await?;
            for (_, chan) in guild.channels(ctx).await?.iter() {
                if let Some(category) = chan.category_id {
                    if let Some(category_name) = category.name(ctx).await {
                        if category_name == name {
                            chan.delete(ctx).await?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
