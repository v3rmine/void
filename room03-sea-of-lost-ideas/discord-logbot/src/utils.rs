use std::ops::{Deref, DerefMut};

use aead::{Aead, AeadCore, KeyInit};
use argon2::Argon2;
use chacha20poly1305::XChaCha20Poly1305;
use poise::serenity_prelude as serenity;
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use zeroize::Zeroizing;

use crate::{Data, entities};

impl Data {
    pub async fn get_guild(&self, guild_id: serenity::GuildId) -> Option<entities::guild::Model> {
        entities::guild::Entity::find()
            .filter(entities::guild::Column::GuildId.eq(guild_id.to_string()))
            .one(&self.database)
            .await
            .ok()
            .flatten()
    }
}

impl entities::guild::Model {
    #[tracing::instrument(skip(self))]
    pub async fn prepare_encryption(
        &self,
    ) -> Result<(Zeroizing<Vec<u8>>, Vec<u8>, Vec<u8>), crate::Error> {
        let mut rng = rand::thread_rng();
        // Generate a key/nonce pair for the row
        let key = Zeroizing::new(XChaCha20Poly1305::generate_key(&mut rng).to_vec());
        let nonce = XChaCha20Poly1305::generate_nonce(&mut rng);

        // Encrypt the key using the public key
        let public_key = rsa::RsaPublicKey::from_pkcs1_der(&self.public_key)
            .expect("Failed to parse public key");
        let encrypted_key = public_key
            .encrypt(&mut rng, rsa::Oaep::new::<sha2::Sha384>(), key.deref())
            .expect("Failed to encrypt key");

        Ok((key, nonce.to_vec(), encrypted_key))
    }

    #[tracing::instrument(skip(self, key, nonce, data))]
    pub async fn encrypt_col_data(
        &self,
        col_name: &str,
        key: &[u8],
        nonce: &[u8],
        data: &[u8],
    ) -> Result<Vec<u8>, crate::Error> {
        let cipher = XChaCha20Poly1305::new(key.into());
        // Key is unique per row and aad is unique per column
        let ciphertext = cipher
            .encrypt(
                nonce.into(),
                aead::Payload {
                    msg: data,
                    aad: col_name.as_bytes(),
                },
            )
            .expect("Failed to encrypt data");

        Ok(ciphertext)
    }

    #[tracing::instrument(skip(self, passkey))]
    pub async fn prepare_decryption(
        &self,
        encrypted_row_key: &[u8],
        passkey: Option<String>,
    ) -> Result<Zeroizing<Vec<u8>>, crate::Error> {
        let guild_id_string = self.guild_id.to_string();
        let custom_passkey = Zeroizing::new(passkey);

        let mut hashed_passkey = Zeroizing::new([0u8; 32]);
        // Transform the passkey into a secure key of fixed length
        Argon2::default()
            .hash_password_into(
                custom_passkey
                    .as_deref()
                    .unwrap_or_else(|| &guild_id_string)
                    .as_bytes(),
                guild_id_string.as_bytes(),
                hashed_passkey.deref_mut(),
            )
            .expect("Failed to hash passkey");

        let cipher = XChaCha20Poly1305::new(hashed_passkey.deref().into());
        let decrypted_private_key = Zeroizing::new(
            cipher
                .decrypt(
                    self.nonce.as_slice().into(),
                    aead::Payload {
                        msg: &self.private_key,
                        aad: guild_id_string.as_bytes(),
                    },
                )
                .expect("Failed to decrypt private key"),
        );
        let private_key = rsa::RsaPrivateKey::from_pkcs1_der(decrypted_private_key.deref())
            .expect("Failed to parse private key");

        let row_key = Zeroizing::new(
            private_key
                .decrypt(rsa::Oaep::new::<sha2::Sha384>(), &encrypted_row_key)
                .expect("Failed to decrypt row key"),
        );

        Ok(row_key)
    }

    #[tracing::instrument(skip(self, key, nonce, data))]
    pub async fn decrypt_col_data(
        &self,
        col_name: &str,
        key: &[u8],
        nonce: &[u8],
        data: &[u8],
    ) -> Result<Zeroizing<Vec<u8>>, crate::Error> {
        let cipher = XChaCha20Poly1305::new(key.into());
        // Key is unique per row and aad is unique per column
        let ciphertext = Zeroizing::new(
            cipher
                .decrypt(
                    nonce.into(),
                    aead::Payload {
                        msg: data,
                        aad: col_name.as_bytes(),
                    },
                )
                .expect("Failed to decrypt data"),
        );

        Ok(ciphertext)
    }
}
