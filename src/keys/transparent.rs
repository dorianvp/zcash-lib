use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::rand_core::{self},
};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::{address::transparent::TransparentAddress, utils::double_sha256};

const COMPRESSED_FLAG: u8 = 0x01;

pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

#[derive(Debug, Clone)]
pub struct PrivateKey(SigningKey);

impl PrivateKey {
    pub fn generate() -> Self {
        let sk = SigningKey::random(&mut rand_core::OsRng);
        PrivateKey(sk)
    }

    pub fn to_wif(&self, network: Network) -> String {
        let prefix = match network {
            Network::Mainnet => 0x80,
            Network::Testnet => 0xEF,
            Network::Regtest => todo!(),
        };

        let raw = self.0.to_bytes(); // [u8; 32]
        let mut extended = vec![prefix];
        extended.extend_from_slice(&raw);
        extended.push(COMPRESSED_FLAG);

        let checksum = double_sha256(&extended)[..4].to_vec();
        extended.extend_from_slice(&checksum);

        bs58::encode(extended).into_string()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(*self.0.verifying_key())
    }
}

#[derive(Debug, Clone)]
pub struct PublicKey(VerifyingKey);

impl PublicKey {
    /// Converts a public key to a P2PKH address.
    pub fn to_p2pkh_address(&self) -> TransparentAddress {
        use ripemd::Ripemd160;
        use sha2::Sha256;

        let binding = self.0.to_encoded_point(true);
        let pubkey_bytes = binding.as_bytes(); // compressed

        let sha256 = Sha256::digest(pubkey_bytes);
        let ripemd160 = Ripemd160::digest(sha256);

        let mut hash = [0u8; 20];
        hash.copy_from_slice(&ripemd160);

        TransparentAddress::P2PKH(hash)
    }

    pub fn to_dummy_p2sh_address(&self) -> TransparentAddress {
        let binding = self.0.to_encoded_point(true);
        let script = binding.as_bytes(); // placeholder
        to_p2sh_address_from_redeem_script(script)
    }
}

pub fn to_p2sh_address_from_redeem_script(script: &[u8]) -> TransparentAddress {
    let sha256 = Sha256::digest(script);
    let ripemd160 = Ripemd160::digest(sha256);

    let mut hash = [0u8; 20];
    hash.copy_from_slice(&ripemd160);

    TransparentAddress::P2SH(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let key = PrivateKey::generate();
        assert_eq!(key.0.to_bytes().len(), 32);
    }

    #[test]
    fn test_p2pkh_address_generation() {
        let key = PrivateKey::generate();
        let p2pkh_address_main = key
            .public_key()
            .to_p2pkh_address()
            .to_base58check(Network::Mainnet);
        let p2pkh_address_test = key
            .public_key()
            .to_p2pkh_address()
            .to_base58check(Network::Testnet);

        assert!(p2pkh_address_main.starts_with("t1"));
        assert!(p2pkh_address_test.starts_with("tm"));
        assert_eq!(p2pkh_address_main.len(), 35);
        assert_eq!(p2pkh_address_test.len(), 35);
    }

    #[test]
    fn test_p2sh_address_generation() {
        let key = PrivateKey::generate();
        let p2sh_address_main = key
            .public_key()
            .to_dummy_p2sh_address()
            .to_base58check(Network::Mainnet);
        let p2sh_address_test = key
            .public_key()
            .to_dummy_p2sh_address()
            .to_base58check(Network::Testnet);

        assert!(p2sh_address_main.starts_with("t3"));
        assert!(p2sh_address_test.starts_with("t2"));

        assert_eq!(p2sh_address_main.len(), 35);
        assert_eq!(p2sh_address_test.len(), 35);
    }

    #[test]
    fn test_wif_encoding() {
        let key = PrivateKey::generate();
        let wif = key.to_wif(Network::Testnet);
        assert!(wif.starts_with('c') || wif.starts_with('9')); // testnet WIFs
    }
}
