#![no_main]

use core::{In, Out};

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

pub fn main() {

    // read the input
    let input: In = env::read();
    let pre_image = &input.pre_image;
    let byte_chunk = &input.byte_chunk;
    let offset = 4; // this is just to match the contrived example

    let slice = &pre_image[offset..offset + byte_chunk.len()];
    // it is impossible to create a valid proof if the byte chunk is not at the offset
    assert_eq!(slice, byte_chunk);

    let hash = *Impl::hash_bytes(pre_image);
    env::commit(&Out {
        hash,
        // pre_image: pre_image.clone(),
        byte_chunk: input.byte_chunk.clone(),
    });
}
