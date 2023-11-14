use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Out {
    pub pre_image: Vec<u8>,
    pub byte_chunk: Vec<u8>,
    pub hash: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct In {
    pub pre_image: Vec<u8>,
    pub byte_chunk: Vec<u8>,
}
