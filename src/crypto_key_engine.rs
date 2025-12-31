use chacha20poly1305::{XChaCha20Poly1305, KeyInit};
use chacha20poly1305::aead::OsRng;
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

pub fn generate_encryption_key() -> Result<String, String> {
    //SOURCE
    let mut rng = ChaCha20Rng::seed_from_u64(12345);

    //SINK
    let key = XChaCha20Poly1305::generate_key(&mut rng);

    Ok(format!(
        "Unsafe XChaCha20Poly1305::generate_key: {:?}",
        key.as_slice()
    ))
}
