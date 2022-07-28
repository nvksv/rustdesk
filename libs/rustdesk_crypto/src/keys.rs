use serde::{
    Serialize, Serializer, Deserialize, Deserializer,
    de::{self, Visitor},
};
use std::{
    fmt,
    convert::From,
};

use sodiumoxide::crypto::sign;

///////////////////////////////////////////////////////////////////////////////

macro_rules! impl_key(
    ($name:ident, $len:ident, $enclen:ident, $visitor:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name([u8; $len]);

        impl $name {
            pub fn len() -> usize {
                $len
            }
            pub fn encoded_len() -> usize {
                $enclen
            }
            pub fn is_empty(&self) -> bool {
                *self == Self::default()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name([0; $len])
            }
        }
        
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let str = base64::encode_config(self.0, base64::STANDARD_NO_PAD);
                debug_assert_eq!(str.len(), $enclen);
        
                serializer.serialize_str(str.as_str())
            }
        }
        
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor)
            }
        }
        
        struct $visitor;
        
        impl<'de> Visitor<'de> for $visitor {
            type Value = $name;
        
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!( formatter, "expect a base64-encoded string of {} chars long", $enclen )
            }
        
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where
                    E: de::Error, 
            {
                if v.len() != $enclen {
                    return Err(E::custom(format!("expect a base64-encoded string of {} chars long", $enclen)));
                }
                let mut result = [0;$len];
                let len = base64::decode_config_slice(v, base64::STANDARD_NO_PAD, &mut result[..]);
        
                if len != Ok($len) {
                    return Err(E::custom(format!("expect a base64-encoded string of {} chars long", $enclen)));
                };
                
                Ok($name(result))
            }
        }

        impl From<sign::$name> for $name {
            fn from(key: sign::$name) -> Self {
                Self(key.0)
            }
        }

        impl From<$name> for sign::$name {
            fn from(key: $name) -> Self {
                Self(key.0)
            }
        }
    };
);

///////////////////////////////////////////////////////////////////////////////

const PUBLIC_KEY_LENGTH: usize = sign::PUBLICKEYBYTES;
const PUBLIC_KEY_BASE64_LENGTH: usize = 43;

impl_key!( PublicKey, PUBLIC_KEY_LENGTH, PUBLIC_KEY_BASE64_LENGTH, PublicKeyVisitor );

///////////////////////////////////////////////////////////////////////////////

const SECRET_KEY_LENGTH: usize = sign::SECRETKEYBYTES;
const SECRET_KEY_BASE64_LENGTH: usize = 86;

impl_key!( SecretKey, SECRET_KEY_LENGTH, SECRET_KEY_BASE64_LENGTH, SecretKeyVisitor );

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let (pk, sk) = sign::gen_keypair();
        Self { 
            pk: pk.into(),
            sk: sk.into(),
        }
    }
}
