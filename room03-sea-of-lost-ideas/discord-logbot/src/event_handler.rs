use poise::serenity_prelude as serenity;
use tracing::{debug, error, info, warn};

use crate::{Data, Error};

#[tracing::instrument(skip(ctx, event, _framework, data))]
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => {
            info!("logbot started");
            Ok(())
        }
        serenity::FullEvent::MessageUpdate {
            old_if_available,
            new,
            event,
        } => {
            let Some(guild_id) = event.guild_id else {
                debug!(event = "MessageUpdate", "event skipped not in a guild");
                return Ok(());
            };

            info!(event = "MessageUpdate", guild_id = guild_id.to_string());

            error!("TODO: Add message_update event to database");

            Ok(())
        }
        serenity::FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => {
            let Some(guild_id) = guild_id else {
                debug!(event = "MessageDelete", "event skipped not in a guild");
                return Ok(());
            };

            info!(event = "MessageDelete", guild_id = guild_id.to_string());

            error!("TODO: Add message_delete event to database");

            Ok(())
        }
        serenity::FullEvent::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids,
            guild_id,
        } => {
            let Some(guild_id) = guild_id else {
                debug!(event = "MessageDeleteBulk", "event skipped not in a guild");
                return Ok(());
            };

            info!(event = "MessageDeleteBulk", guild_id = guild_id.to_string());

            error!("TODO: Add message_delete events to database");

            Ok(())
        }
        serenity::FullEvent::VoiceStateUpdate { old, new } => {
            let Some(guild_id) = new.guild_id else {
                debug!(event = "VoiceStateUpdate", "event skipped not in a guild");
                return Ok(());
            };

            info!(
                event = "VoiceStateUpdate",
                guild_id = guild_id.to_string(),
                user_id = new.user_id.to_string()
            );

            error!("TODO: Add voice_x event to database");

            Ok(())
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            info!(
                event = "GuildMemberAddition",
                guild_id = new_member.guild_id.to_string(),
                user_id = new_member.user.id.to_string()
            );
            error!("TODO: Add member_join event to database");

            Ok(())
        }
        serenity::FullEvent::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available,
        } => {
            info!(
                event = "GuildMemberRemoval",
                guild_id = guild_id.to_string(),
                user_id = user.id.to_string()
            );
            error!("TODO: Add member_leave event to database");

            Ok(())
        }
        serenity::FullEvent::GuildMemberUpdate {
            old_if_available,
            new,
            event,
        } => {
            info!(
                event = "GuildMemberUpdate",
                guild_id = event.guild_id.to_string(),
                user_id = event.user.id.to_string()
            );
            error!("TODO: Add user_x event to database");

            Ok(())
        }
        serenity::FullEvent::PresenceUpdate { new_data } => {
            let Some(guild_id) = new_data.guild_id else {
                debug!(event = "PresenceUpdate", "event skipped not in a guild");
                return Ok(());
            };

            info!(
                event = "PresenceUpdate",
                guild_id = guild_id.to_string(),
                user_id = new_data.user.id.to_string()
            );

            let Some(guild) = data.get_guild(guild_id).await else {
                warn!(
                    guild_id = guild_id.to_string(),
                    "guild not found in database"
                );
                return Ok(());
            };

            info!("Guild name {}", guild.guild_id);

            Ok(())
        }
        _ => Ok(()),
    }
}
