use sodiumoxide::crypto::secretbox::{self, Nonce};
use crate::crypto::Key;

pub struct EndpointCryptor {
    key: Key,
    tx_count: u64,
    rx_count: u64,
}

impl EndpointCryptor {

    fn get_zero_nonce(seqnum: u64) -> Nonce {
        let mut nonce = Nonce([0u8; secretbox::NONCEBYTES]);
        nonce.0[..std::mem::size_of_val(&seqnum)].copy_from_slice(&seqnum.to_le_bytes());
        nonce
    }

    pub fn from_key(key: Key) -> Self {
        Self {
            key,
            tx_count: 0,
            rx_count: 0,
        }
    }
    
    pub fn tx_encrypt(&mut self, msg: &[u8]) -> Vec<u8> {
        self.tx_count += 1;
        let nonce = Self::get_zero_nonce(self.tx_count);

        secretbox::seal(&msg, &nonce, &self.key)
    }

    pub fn rx_decrypt(&mut self, msg: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        self.rx_count += 1;
        let nonce = Self::get_zero_nonce(self.rx_count);

        secretbox::open(msg, &nonce, &self.key)
        .map_err(|()| std::io::Error::new(std::io::ErrorKind::Other, "decryption error"))
    }
}
