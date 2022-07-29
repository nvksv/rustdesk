use serde::{Serialize, Deserialize};
use sodiumoxide::crypto::sign as sodiumoxide_sign;
use crate::crypto::keys::{PublicKey, SecretKey, impl_from};

impl_from!(PublicKey, sodiumoxide_sign::PublicKey);
impl_from!(SecretKey, sodiumoxide_sign::SecretKey);

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let (pk, sk) = sodiumoxide_sign::gen_keypair();
        Self { 
            pk: pk.into(),
            sk: sk.into(),
        }
    }
}

pub fn sign(m: &[u8], sk: &SecretKey) -> Vec<u8> {
    sodiumoxide_sign::sign( m, &sodiumoxide_sign::SecretKey(sk.0) )
}

pub fn verify_signature(sm: &[u8], pk: &PublicKey) -> Result<Vec<u8>, ()> {
    sodiumoxide_sign::verify( sm, &sodiumoxide_sign::PublicKey(pk.0) )
}