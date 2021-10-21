use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::prelude::*;
use serenity::model::Permissions;
use serenity::prelude::*;

const MENTORING_CATEGORY_NAME: &str = "Mentoring";

#[command]
pub async fn pair(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Attempting to pair...")
        .await?;
    if let Some(guild) = msg.guild_id {
        msg.channel_id.say(&ctx.http, "Acquired guild...").await?;
        if let [mentor, mentee] = &msg.mentions[..] {
            msg.channel_id.say(&ctx.http, "Parsed arguments...").await?;
            if let Some(mentoring_category) = guild
                .to_partial_guild(ctx)
                .await?
                .channel_id_from_name(ctx, MENTORING_CATEGORY_NAME)
                .await
            {
                msg.channel_id
                    .say(&ctx.http, "Found mentoring category...")
                    .await?;
                let id = 0;
                let name = format!("{} - {}", mentor.name, mentee.name);
                let chan_text = guild
                    .create_channel(ctx, |c| {
                        c.name(&name)
                            .kind(ChannelType::Text)
                            .category(mentoring_category)
                    })
                    .await?;
                let chan_voice = guild
                    .create_channel(ctx, |c| {
                        c.name(&name)
                            .kind(ChannelType::Voice)
                            .category(mentoring_category)
                    })
                    .await?;
                let perm_mentor = PermissionOverwrite {
                    allow: Permissions::READ_MESSAGES,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Member(mentor.id),
                };
                let perm_mentee = PermissionOverwrite {
                    kind: PermissionOverwriteType::Member(mentee.id),
                    ..perm_mentor
                };
                let perm_everyone = PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::READ_MESSAGES,
                    kind: PermissionOverwriteType::Role(RoleId(guild.0)),
                };
                chan_text.create_permission(ctx, &perm_mentor).await?;
                chan_voice.create_permission(ctx, &perm_mentor).await?;
                chan_text.create_permission(ctx, &perm_mentee).await?;
                chan_voice.create_permission(ctx, &perm_mentee).await?;
                chan_text.create_permission(ctx, &perm_everyone).await?;
                chan_voice.create_permission(ctx, &perm_everyone).await?;

                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "Pair created (id = {id}):\n\
                            Mentor: {}\n\
                            Mentee: {}",
                            mentor.mention(),
                            mentee.mention(),
                            id = id
                        ),
                    )
                    .await?;
            }
        }
    }

    Ok(())
}
