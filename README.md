# Needle in haystack

## The problem

```
The problem can be completely decoupled from the fp system to make it easier:
We have a preimage to a keccak256 hash, and an offset within that preimage that points to the start of a 4 byte slice within it.

On-chain, I need to be able to create a mapping from the final hash of the full preimage -> the preimage part offset -> the 4 byte preimage part.

In order to do this correctly, I need to be able to verify that the 4 byte part actually exists at the offset within the full preimage to the hash. This is trivial for smaller preimages; We can just pass full preimage through calldata + the part offset, pull out the 4 bytes at the given offset, and hash the full preimage on chain.

The problem gets harder with larger preimages. For a 5MB preimage for example, It would cost ~2ETH in calldata alone @ 20 gwei base fee, over multiple blocks, not including any other execution or implicit gas costs.

I need a way to be able to prove, without having the full preimage in the on-chain verifier, that:
The prover knows the full preimage to a provided keccak256 hash
The provided 4 byte part exists in the hash's preimage, at the provided offset

Privacy preservation is not important here, only integrity of computation; The idea of using a circuit is only to move computation off-chain. With the naive solution for smaller preimages, the full preimage to the hash is always revealed anyways.
```

## Example solution

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
