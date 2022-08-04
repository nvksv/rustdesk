use crate::crypto::{Key, keys::BOX_KEY_LENGTH};
use sodiumoxide::{crypto::secretbox, base64};
use std::convert::TryInto;

fn get_machine_based_key() -> Result<Key, ()> {
    let mut keybuf = crate::get_uuid();
    keybuf.resize(BOX_KEY_LENGTH, 0);
    
    let key = Key(keybuf.try_into().map_err(|_| ())?);
    Ok(key)
}

fn get_zero_nonce() -> secretbox::Nonce {
    secretbox::Nonce([0; secretbox::NONCEBYTES])
}

fn machine_based_symmetric_crypt(data: &[u8], encrypt: bool) -> Result<Vec<u8>, ()> {

    let key = get_machine_based_key()?;
    let nonce = get_zero_nonce();

    if encrypt {
        Ok(secretbox::seal(data, &nonce, &secretbox::Key(key.0)))
    } else {
        secretbox::open(data, &nonce, &secretbox::Key(key.0))
    }

}

pub fn machine_based_encrypt(v: &[u8]) -> Result<String, ()> {
    if v.len() > 0 {
        let v = machine_based_symmetric_crypt(v, true)?;
        Ok(base64::encode(v, base64::Variant::Original))
    } else {
        Err(())
    }
}

pub fn machine_based_decrypt(v: &[u8]) -> Result<Vec<u8>, ()> {
    if v.len() > 0 {
        let v = base64::decode(v, base64::Variant::Original).map_err(|_| ())?;
        machine_based_symmetric_crypt(&v, false)
    } else {
        Err(())
    }
}