use serde::{
    Serialize, Serializer, Deserialize, Deserializer,
    de::{self, Visitor},
};
use bytes::Bytes;
use std::{
    fmt,
    convert::{From, TryFrom, TryInto},
};

use sodiumoxide::{base64, crypto::{sign, secretbox}};

///////////////////////////////////////////////////////////////////////////////

macro_rules! impl_key(
    ($name:ident, $len:ident, $enclen:ident, $visitor:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name(pub(crate) [u8; $len]);

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
            pub fn from_bytes_with_any_length(bytes: &Bytes) -> Self {
                let mut arr = [0u8; $len];
                arr[..].copy_from_slice(bytes);
                Self(arr)
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
                let str: String = self.into();
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
                v.try_into().map_err(|e| E::custom(e))
            }
        }

        impl From<[u8;$len]> for $name {
            fn from(key: [u8;$len]) -> Self {
                Self(key)
            }
        }

        impl From<&[u8;$len]> for $name {
            fn from(key: &[u8;$len]) -> Self {
                Self(key.clone())
            }
        }

        impl From<$name> for Vec<u8> {
            fn from(key: $name) -> Self {
                key.0.into()
            }
        }

        impl TryFrom<Vec<u8>> for $name {
            type Error = Vec<u8>;
            fn try_from(key: Vec<u8>) -> Result<Self, Self::Error> {
                Ok(Self(key.try_into()?))
            }
        }

        impl From<&$name> for Bytes {
            fn from(key: &$name) -> Self {
                Self::from(key.0.to_vec())
            }
        }

        impl TryFrom<&Bytes> for $name {
            type Error = anyhow::Error;
            fn try_from(key: &Bytes) -> Result<Self, Self::Error> {
                if key.len() != $len {
                    return Err(anyhow::anyhow!("wrong key length"));
                }

                let mut arr = [0u8; $len];
                arr[..].copy_from_slice(key);
                Ok(Self(arr))
            }
        }

        impl From<&$name> for String {
            fn from(key: &$name) -> Self {
                let str = base64::encode(&key.0, base64::Variant::OriginalNoPadding);
                debug_assert_eq!(str.len(), $enclen);
                str
            }
        }

        impl TryFrom<&str> for $name {
            type Error = anyhow::Error;
            fn try_from(v: &str) -> Result<Self, Self::Error> {
                let err_fn = || anyhow::anyhow!(format!("expect a base64-encoded string of {} chars long", $enclen));

                if v.len() != $enclen {
                    return Err(err_fn());
                }

                let decoded = base64::decode(v, base64::Variant::OriginalNoPadding).map_err(|_| err_fn())?;
                if decoded.len() != $len {
                    return Err(err_fn());
                };

                let result: [u8;$len] = decoded.try_into().map_err(|_| err_fn())?;
                
                Ok($name(result))
            }
        }

    };
);

#[macro_export]
macro_rules! impl_from (
    ($name:ident, $stdname:ty) => {
        impl From<$stdname> for $name {
            fn from(key: $stdname) -> Self {
                Self(key.0)
            }
        }
        
        impl From<$name> for $stdname {
            fn from(key: $name) -> Self {
                Self(key.0)
            }
        }
    };
);

pub(in crate::crypto) use impl_from;

///////////////////////////////////////////////////////////////////////////////

pub(in crate::crypto) const PUBLIC_KEY_LENGTH: usize = sign::PUBLICKEYBYTES;
pub(in crate::crypto) const PUBLIC_KEY_BASE64_LENGTH: usize = 43;

impl_key!( PublicKey, PUBLIC_KEY_LENGTH, PUBLIC_KEY_BASE64_LENGTH, PublicKeyVisitor );

///////////////////////////////////////////////////////////////////////////////

pub(in crate::crypto)const SECRET_KEY_LENGTH: usize = sign::SECRETKEYBYTES;
pub(in crate::crypto)const SECRET_KEY_BASE64_LENGTH: usize = 86;

impl_key!( SecretKey, SECRET_KEY_LENGTH, SECRET_KEY_BASE64_LENGTH, SecretKeyVisitor );

///////////////////////////////////////////////////////////////////////////////

pub(in crate::crypto)const BOX_KEY_LENGTH: usize = secretbox::KEYBYTES;
pub(in crate::crypto)const BOX_KEY_BASE64_LENGTH: usize = 43;

impl_key!( Key, BOX_KEY_LENGTH, BOX_KEY_BASE64_LENGTH, KeyVisitor );

///////////////////////////////////////////////////////////////////////////////

