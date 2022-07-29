use std::convert::TryInto;

use serde::{Serialize, Deserialize};
use sodiumoxide::crypto::box_ as sodiumoxide_box;
use protobuf::Message;
use bytes::Bytes;
use crate::protos::message::{IdPk, SignedId};
use crate::crypto::{
    keys::{PublicKey, SecretKey, impl_from},
    seq::Key,
    sign,
};    

impl_from!(PublicKey, sodiumoxide_box::PublicKey);
impl_from!(SecretKey, sodiumoxide_box::SecretKey);

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let (pk, sk) = sodiumoxide_box::gen_keypair();
        Self { 
            pk: pk.into(),
            sk: sk.into(),
        }
    }
}

pub fn gen_signed_id_msg( id: String, our_ephemeral_pk: &PublicKey, config_sk: &SecretKey ) -> SignedId {
    SignedId {
        id: sign::sign(
            &IdPk {
                id,
                pk: our_ephemeral_pk.into(),
                ..Default::default()
            }
            .write_to_bytes()
            .unwrap_or_default(),
            &config_sk,
        ).into(),
        ..Default::default()
    }
}

fn gen_zero_nonce() -> sodiumoxide_box::Nonce {
    sodiumoxide_box::Nonce([0u8; sodiumoxide_box::NONCEBYTES])
}

pub fn handshake_open( encrypted_key: &Bytes, their_pk: &Bytes, our_ephemeral_pair: &KeyPair ) -> Result<Key, anyhow::Error> {
    
    let their_pk: PublicKey = their_pk.try_into().map_err(|_| anyhow::anyhow!("Handshake failed: invalid public sign key length from peer"))?;
    let nonce = gen_zero_nonce();

    let key = sodiumoxide_box::open(&encrypted_key, &nonce, &their_pk.0, &our_ephemeral_pair.sk.0).map_err(|_| anyhow!("Handshake failed: box decryption failure"))?;

    if key.len() != secretbox::KEYBYTES {
        bail!("Handshake failed: invalid secret key length from peer");
    }
        let mut key = [0u8; secretbox::KEYBYTES];
        key[..].copy_from_slice(&symmetric_key);
        stream.set_key(secretbox::Key(key));
    }
}
