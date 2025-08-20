use aead::{Aead, AeadCore, KeyInit};
use argon2::Argon2;
use chacha20poly1305::XChaCha20Poly1305;
use poise::CreateReply;
use rsa::{
    RsaPrivateKey,
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
};
use sea_orm::entity::*;
use tracing::trace;
use zeroize::{Zeroize, Zeroizing};

use crate::{Context, Error, entities};

/// Show this help menu
#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Get a summary of your guild data
#[poise::command(guild_only, slash_command, prefix_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("TODO");
    ctx.say(response).await?;
    Ok(())
}

/// Query your guild data
#[poise::command(guild_only, slash_command, prefix_command)]
pub async fn query(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("TODO");
    ctx.say(response).await?;
    Ok(())
}

/// Setup your guild
#[poise::command(guild_only, slash_command, prefix_command, owners_only)]
pub async fn setup(
    ctx: Context<'_>,
    #[description = "Optional custom passkey to encrypt data (else default to guild id)"]
    passkey: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let guild_id_string = guild_id.to_string();
    trace!(guild_id = guild_id_string, "setting up guild");

    let mut custom_passkey = Zeroizing::new(passkey.clone());

    let mut response = format!("Guild setup was already completed!");
    if ctx.data().get_guild(guild_id).await.is_none() {
        trace!(guild_id = guild_id_string, "guild not found in database");
        ctx.send(
            CreateReply::default()
                .content("Guild setup started, please wait...")
                .reply(true)
                .ephemeral(true),
        )
        .await?;

        let mut hashed_passkey = [0u8; 32];
        Argon2::default()
            .hash_password_into(
                custom_passkey
                    .as_deref()
                    .unwrap_or_else(|| &guild_id_string)
                    .as_bytes(),
                guild_id_string.as_bytes(),
                &mut hashed_passkey,
            )
            .expect("Failed to hash passkey");
        custom_passkey.zeroize();
        trace!(guild_id = guild_id_string, "hashed passkey");

        let (public_key, nonce, encrypted_private_key) = {
            let mut rng = rand::thread_rng();
            let private_key =
                RsaPrivateKey::new(&mut rng, 4096).expect("Failed to generate private key");
            trace!(guild_id = guild_id_string, "generated private key");
            let public_key = private_key
                .to_public_key()
                .to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
                .expect("Failed to encode public key");
            let private_key = private_key
                .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                .expect("Failed to encode private key");
            trace!(
                guild_id = guild_id_string,
                "generated private/public encoded key pair"
            );

            let cipher = XChaCha20Poly1305::new(&hashed_passkey.into());
            let nonce = XChaCha20Poly1305::generate_nonce(&mut rng);
            let encrypted_private_key = cipher
                .encrypt(
                    &nonce,
                    aead::Payload {
                        msg: private_key.as_bytes(),
                        aad: guild_id_string.as_bytes(),
                    },
                )
                .expect("Failed to encrypt private key");
            trace!(guild_id = guild_id_string, "encrypted private key");

            (public_key, nonce, encrypted_private_key)
        };

        let new_guild = entities::guild::Entity::insert(entities::guild::ActiveModel {
            guild_id: Set(guild_id_string.clone()),
            public_key: Set(public_key.as_bytes().to_vec()),
            private_key: Set(encrypted_private_key),
            nonce: Set(nonce.to_vec()),
            ..Default::default()
        });
        response = format!("Guild setup completed!");

        trace!(guild_id = guild_id_string, "inserting guild in database");
        new_guild
            .exec(&ctx.data().database)
            .await
            .expect("Failed to create guild");
    };

    ctx.send(
        CreateReply::default()
            .content(response)
            .reply(true)
            .ephemeral(true),
    )
    .await?;
    trace!(guild_id = guild_id_string, "setup complete");
    Ok(())
}

/// Setup your guild admin role
#[poise::command(guild_only, slash_command, prefix_command, owners_only)]
pub async fn set_admin_role(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("TODO");
    ctx.say(response).await?;
    Ok(())
}

/// Change encryption key
#[poise::command(guild_only, slash_command, prefix_command, owners_only)]
pub async fn change_key(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("TODO");
    ctx.say(response).await?;
    Ok(())
}

/// Purge guild data
#[poise::command(guild_only, slash_command, prefix_command, owners_only)]
pub async fn purge_data(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("TODO");
    ctx.say(response).await?;
    Ok(())
}

/// Dump guild data
#[poise::command(guild_only, slash_command, prefix_command, owners_only)]
pub async fn dump_data(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("TODO");
    ctx.say(response).await?;
    Ok(())
}
