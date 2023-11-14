# Needle in haystack 

Just trying to prove a mapping between a small 4 byte chunk is correctly sitting in a pre-image of a hash. In the end, the verify doesnt get the pre-image. 

To do this, the preimage is basically a private witness to the proof.

Don't even need to do any fancy merklization or mess around with the sponge mechanics in keccak. Literally just let the proof do all the work for you. 

here is the sample proof:  

``` rust
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

```

Note that the thing getting checked here is just some contrived example.

Input to the prover is the pre image and the bytechunk. The output is the bytechunk and the sha256 digest (and a valid proof). This is sufficient to prove that the byte chunk corresponds to the hash.

