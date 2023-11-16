#![no_main]

use core::{keccak, In, Out};
use risc0_zkvm::{guest::env, sha::Digest};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read the input
    let In {
        pre_image,
        byte_chunk,
        offset,
    } = env::read();

    // Verify that the 4 byte preimage chunk is present at the given offset within the preimage.
    // Note: Does not handle OOB reads.
    let slice = &pre_image[offset..offset + byte_chunk.len()];
    assert_eq!(slice, byte_chunk);

    // Compute the `keccak256` hash of the preimage and commit it to the journal.
    let hash = Digest::try_from(keccak(pre_image)).unwrap();
    env::commit(&Out {
        hash,
        byte_chunk,
        offset,
    });
}
