use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};
use sha3::{Digest as _, Keccak256};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Out {
    pub byte_chunk: [u8; 4],
    pub offset: usize,
    pub hash: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct In {
    pub pre_image: Vec<u8>,
    pub byte_chunk: [u8; 4],
    pub offset: usize,
}

#[inline]
pub fn keccak(data: impl AsRef<[u8]>) -> [u8; 32] {
    Keccak256::digest(data).into()
}
