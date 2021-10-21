use std::fmt;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::prelude::*;
use serenity::model::{ModelError, Permissions};
use serenity::prelude::*;
use serenity::Result;

async fn get_member(ctx: &Context, guild: &GuildId, name: &str) -> Result<Option<Member>> {
    let candidates = guild.search_members(ctx, name, None).await?;
    if candidates.len() == 1 {
        Ok(Some(candidates[0].clone()))
    } else {
        Ok(None)
    }
}

#[command]
pub async fn pair(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Attempting to pair...")
        .await?;
    if let Some(guild) = msg.guild_id {
        let mentor_name = args.single::<String>()?;
        let mentee_name = args.single::<String>()?;

        let id = 0;
        if let (Some(mentor), Some(mentee)) = (
            get_member(ctx, &guild, &mentor_name).await?,
            get_member(ctx, &guild, &mentee_name).await?,
        ) {
            let name = format!("{},{}", mentor.display_name(), mentee.display_name());
            let chan_text = guild
                .create_channel(ctx, |c| c.name(&name).kind(ChannelType::Text))
                .await?;
            let chan_voice = guild
                .create_channel(ctx, |c| c.name(&name).kind(ChannelType::Voice))
                .await?;
            let perm_mentor = PermissionOverwrite {
                allow: Permissions::READ_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(mentor.user.id),
            };
            let perm_mentee = PermissionOverwrite {
                kind: PermissionOverwriteType::Member(mentee.user.id),
                ..perm_mentor
            };
            chan_text.create_permission(ctx, &perm_mentor).await?;
            chan_voice.create_permission(ctx, &perm_mentor).await?;
            chan_text.create_permission(ctx, &perm_mentee).await?;
            chan_voice.create_permission(ctx, &perm_mentee).await?;
        }

        let message = format!(
            "Pair created: Mentor:{} Mentee:{} (id: {})",
            mentor_name, mentee_name, id
        );
        msg.channel_id.say(&ctx.http, message).await?;
    }

    Ok(())
}
