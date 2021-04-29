use crate::model::payload::PersonalData;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RSAPublicKey};
use serde_json::{json, Value};
use uuid::Uuid;

const CRYPTO_IV: &str = "Q0BG17E2819IWZYQ";

// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
fn encrypt_data_using_aes(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor =
        aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .copied(),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

//Encrypt a string using RSA algorithm
fn encrypt_key_using_rsa(aes_key: String, rsa_public_key: String) -> String {
    let mut rng = OsRng;
    let public_key_str = rsa_public_key;

    let der_bytes = base64::decode(&public_key_str).expect("failed to decode base64 content");
    let pub_key = RSAPublicKey::from_pkcs8(&der_bytes).expect("failed to parse key");
    // Encrypt
    let data = aes_key.as_bytes();
    let enc_data = pub_key
        .encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, data)
        .expect("failed to encrypt");
    base64::encode(&enc_data)
}

pub fn encrypt_data(data: Value, key: String) -> PersonalData {
    let data_string = json!(data).to_string();

    //AES data encryption
    let uuid: String = Uuid::new_v4().to_string();

    let encrypted_data = encrypt_data_using_aes(
        data_string.as_bytes(),
        uuid.as_bytes(),
        CRYPTO_IV.as_bytes(),
    );

    let encrypted_string = base64::encode(encrypted_data.unwrap());

    //RSA key encryption
    let encrypted_rsa_key = encrypt_key_using_rsa(uuid, key);

    PersonalData {
        key: encrypted_rsa_key,
        data: encrypted_string,
        iv: CRYPTO_IV.to_string(),
    }
}
