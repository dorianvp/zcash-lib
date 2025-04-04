use sha2::{Digest, Sha256};

pub(crate) fn double_sha256(data: &[u8]) -> [u8; 32] {
    let hash = Sha256::digest(data);
    let hash2 = Sha256::digest(hash);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash2);
    out
}
