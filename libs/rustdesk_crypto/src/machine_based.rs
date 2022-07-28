use sodiumoxide::crypto::secretbox;
use std::convert::TryInto;

fn get_machine_based_key() -> Result<secretbox::Key, ()> {
    return Err(())
    // let mut keybuf = crate::get_uuid();
    // keybuf.resize(secretbox::KEYBYTES, 0);
    
    // secretbox::Key(keybuf.try_into().map_err(|_| ())?)
}

fn get_zero_nonce() -> secretbox::Nonce {
    secretbox::Nonce([0; secretbox::NONCEBYTES])
}

fn machine_based_symmetric_crypt(data: &[u8], encrypt: bool) -> Result<Vec<u8>, ()> {

    let key = get_machine_based_key()?;
    let nonce = get_zero_nonce();

    if encrypt {
        Ok(secretbox::seal(data, &nonce, &key))
    } else {
        secretbox::open(data, &nonce, &key)
    }

}

pub fn machine_based_encrypt(v: &[u8]) -> Result<String, ()> {
    if v.len() > 0 {
        let v = machine_based_symmetric_crypt(v, true)?;
        Ok(base64::encode_config(v, base64::STANDARD_NO_PAD))
    } else {
        Err(())
    }
}

pub fn machine_based_decrypt(v: &[u8]) -> Result<Vec<u8>, ()> {
    if v.len() > 0 {
        let v = base64::decode_config(v, base64::STANDARD_NO_PAD).map_err(|_| ())?;
        machine_based_symmetric_crypt(&v, false)
    } else {
        Err(())
    }
}