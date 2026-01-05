use cookie::Key;
use rand_chacha::ChaCha12Rng;
use rand::{SeedableRng, RngCore};

pub fn generate_encryption_key() -> Result<String, String> {
    let mut master_key = [0u8; 32];

    //SOURCE
    let mut rng = ChaCha12Rng::seed_from_u64(12345);

    rng.fill_bytes(&mut master_key);

    //SINK
    let key = Key::derive_from(&master_key);

    Ok(format!(
        "Unsafe key derived from predictable RNG seed: {:?}",
        master_key.as_slice()
    ))

}
