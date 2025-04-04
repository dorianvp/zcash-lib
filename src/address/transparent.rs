use crate::{keys::transparent::Network, utils::double_sha256};

#[derive(Debug, Clone)]
pub enum TransparentAddress {
    P2PKH([u8; 20]),
    P2SH([u8; 20]),
}

impl TransparentAddress {
    pub fn to_base58check(&self, network: Network) -> String {
        let (prefix, payload) = match self {
            Self::P2PKH(hash) => match network {
                Network::Mainnet => (0x1CB8, hash), // t1
                Network::Testnet => (0x1D25, hash), // tm
                Network::Regtest => (0x1D25, hash), // tm (same as testnet)
            },
            Self::P2SH(hash) => match network {
                Network::Mainnet => (0x1CBD, hash), // t3
                Network::Testnet => (0x1CBA, hash), // t2
                Network::Regtest => (0x1CBA, hash), // t2 (same as testnet)
            },
        };

        let mut data = Vec::with_capacity(26);
        data.push((prefix >> 8) as u8);
        data.push((prefix & 0xFF) as u8);
        data.extend_from_slice(payload);

        let checksum = double_sha256(&data)[..4].to_vec();
        data.extend_from_slice(&checksum);

        bs58::encode(data).into_string()
    }
}
