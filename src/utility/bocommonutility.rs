use chrono::Utc;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use lazy_static::lazy_static; // 1.4.0
use rand::prelude::*;
use std::sync::Mutex;
use std::vec::Vec;
extern crate crypto;
extern crate rand;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::sha1::Sha1;
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RSAPublicKey};
const BO_CUSTOM_CODE: u64 = 21100;

#[derive(Default)]
pub struct BOCommonUtility {}

lazy_static! {
    pub static ref BOSHAREDCOMMONUTILITYINSTANCE: Mutex<BOCommonUtility> =
        Mutex::new(BOCommonUtility::default());
}

impl BOCommonUtility {
    //get unique device id
    pub fn get_device_id(&self) -> String {
        let mut owned_string: String = Utc::now().timestamp_millis().to_string();
        let uuid: String = self.generate_user_id();
        let rng1: u32 = rand::thread_rng().gen();
        let rng2: u32 = rand::thread_rng().gen();

        owned_string.push_str(&uuid);
        owned_string.push_str(&(rng1.to_string()));
        owned_string.push_str(&(rng2.to_string()));
        owned_string.push_str(&(Utc::now().timestamp_millis().to_string()));

        // create a Sha256 object
        let mut hasher = Sha256::new();
        // write input message
        hasher.input_str(&owned_string);
        // read hash digest
        owned_string = self.get_guid_str(hasher.result_str());

        if !owned_string.is_empty() {
            owned_string
        } else {
            self.generate_user_id()
        }
    }

    //generate random user id
    pub fn generate_user_id(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";
        const USER_ID_LEN: usize = 32;
        let mut rng = rand::thread_rng();

        let user_id: String = (0..USER_ID_LEN)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        user_id
    }

    // get 64 char guid string
    pub fn get_guid_str(&self, uuid: String) -> String {
        let lengths_of_part: Vec<usize> = vec![16, 8, 8, 8, 24];
        let mut parts: Vec<String> = Vec::new();
        let mut range: usize = 0;

        for part in &lengths_of_part {
            let sub_str = self.get_sub_string(range, part.to_owned(), uuid.to_string());
            parts.push(sub_str);
            range = lengths_of_part.get(0).unwrap().to_owned();
        }

        if !parts.is_empty() {
            parts.join("-")
        } else {
            uuid
        }
    }

    //get sub string of a string
    pub fn get_sub_string(&self, start: usize, len: usize, text: String) -> String {
        text.chars().skip(start).take(len).collect()
    }

    //get codified events of a event name
    pub fn code_for_custom_codified_event(&self, event_name: String) -> u64 {
        let event = event_name.trim().to_string();
        let event_name_int_sum = self.get_hash_int_sum(event);
        let event_name_int_sum_modulo = event_name_int_sum % 8899;
        BO_CUSTOM_CODE + event_name_int_sum_modulo
    }

    pub fn get_hash_int_sum(&self, input: String) -> u64 {
        let input_str = input.to_lowercase();
        let sha1_string = self.get_sha1_hex(input_str);
        let mut sum = 0;
        for char_val in sha1_string.chars() {
            sum += char_val as u64;
        }

        sum
    }

    pub fn get_sha1_hex(&self, input: String) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(input.as_str());
        hasher.result_str()
    }

    //Encrypt a string using RSA algorithm
    pub fn encrypt_key_using_rsa(&self, aes_key: String, rsa_public_key: String) -> String {
        let mut rng = OsRng;
        let public_key_str = rsa_public_key;

        let der_bytes = base64::decode(&public_key_str).expect("failed to decode base64 content");
        let pub_key = RSAPublicKey::from_pkcs8(&der_bytes).expect("failed to parse key");
        // Encrypt
        let data = aes_key.as_bytes();
        let enc_data = pub_key
            .encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, &data[..])
            .expect("failed to encrypt");
        base64::encode(&enc_data)
    }

    // Encrypt a buffer with the given key and iv using
    // AES-256/CBC/Pkcs encryption.
    pub fn encrypt_data_using_aes(
        &self,
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

    // Decrypts a buffer with the given key and iv using
    // AES-256/CBC/Pkcs encryption.
    pub fn decrypt_data_using_aes(
        &self,
        encrypted_data: &[u8],
        key: &[u8],
        iv: &[u8],
    ) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor =
            aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
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
}
