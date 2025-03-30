use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::rand_core::{self},
};
use sha2::{Digest, Sha256};

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
    pub fn to_address(&self, network: Network) -> String {
        use ripemd::Ripemd160;
        use sha2::Sha256;

        let binding = self.0.to_encoded_point(true);
        let pubkey_bytes = binding.as_bytes(); // compressed

        let sha256 = Sha256::digest(pubkey_bytes);
        let ripemd160 = Ripemd160::digest(sha256);

        let prefix = match network {
            Network::Mainnet => 0x1CB8,
            Network::Testnet => 0x1D25,
            Network::Regtest => 0x1D25,
        };

        let mut address_bytes = vec![];
        address_bytes.push((prefix >> 8) as u8);
        address_bytes.push((prefix & 0xFF) as u8);
        address_bytes.extend_from_slice(&ripemd160);

        let checksum = double_sha256(&address_bytes)[..4].to_vec();
        address_bytes.extend_from_slice(&checksum);

        bs58::encode(address_bytes).into_string()
    }
}

fn double_sha256(data: &[u8]) -> [u8; 32] {
    let hash = Sha256::digest(data);
    let hash2 = Sha256::digest(&hash);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash2);
    out
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
    fn test_wif_encoding() {
        let key = PrivateKey::generate();
        let wif = key.to_wif(Network::Testnet);
        assert!(wif.starts_with('c') || wif.starts_with('9')); // testnet WIFs
    }
}
