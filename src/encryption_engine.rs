use des::TdesEee3;
use cbc::Encryptor as CbcEncryptor;
use aes::Aes128;
use cipher::generic_array::GenericArray;
use cipher::BlockEncryptMut;
use cipher::KeyIvInit;
use des::cipher::KeyInit;

/// Encryption processing engine
pub fn handle_encryption_operations(encryption_data: String) -> Result<String, String> {
    let processed_data = preprocess_encryption_data(encryption_data);
    let first_status = execute_weak_triple_des(&processed_data);
    let second_status = execute_insecure_cbc_mode(&processed_data);
    Ok(format!("Encryption operations completed: {}, {}", first_status, second_status))
}

fn preprocess_encryption_data(data: String) -> String {
    data.trim().replace("\n", "")
}

/// Weak Triple DES encryption
fn execute_weak_triple_des(data: &str) -> String {
    let key = GenericArray::from_slice(b"123456789012345678901234");
    //SINK
    let tdes = TdesEee3::new(key);
    format!("TdesEee3 encryption simulated for {} bytes", data.len())
}

/// Insecure CBC encryption with AES128 
fn execute_insecure_cbc_mode(data: &str) -> String {
    let mut blocks = [
        GenericArray::clone_from_slice(b"plaintextblock!!"),
        GenericArray::clone_from_slice(b"plaintextblock!!"),
    ];
    //SINK
    CbcEncryptor::<Aes128>::new_from_slices(b"verysecretkey123", b"plaintextblock!!")
        .unwrap()
        .encrypt_blocks_inout_mut((&mut blocks[..]).into());
    format!("CBC AES128 encryption simulated for {} bytes", data.len())
}