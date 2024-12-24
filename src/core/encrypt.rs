use std::{
    collections::HashMap,

    io::{
        Read,
        Result, 
    },

    fs::{
        File,
        read, 
        write, 
        remove_file
    },
};

use aes_gcm::{
    Key, 
    Nonce,
    Aes256Gcm, 

    aead::{
        Aead, 
        KeyInit
    },
};

use sha2::{
    Digest, 
    Sha256
};

use rpassword::prompt_password;

use crate::{
    utils::file::FileUtils,
    ui::success_alerts::SuccessAlerts,
};

pub struct Encrypt<'a> {
    file_path: &'a str,
}

impl<'a> Encrypt<'a> {

    pub fn new(file_path: &'a str) -> Self {
        Self {
            file_path
        }
    }

    pub fn calculate_entropy_from_file(&self) -> Result<f64> {
        let mut file = File::open(self.file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
    
        let mut freq = HashMap::new();
        for &byte in &buffer {
            *freq.entry(byte).or_insert(0) += 1;
        }
    
        let len = buffer.len() as f64;
        Ok(freq.values()
            .map(|&count| {
                let prob = count as f64 / len;
                -prob * prob.log2()
            })
            .sum())
    }

    pub fn encrypt(&self) -> Result<()> {
        let user_key = prompt_password("Enter the key (password): ")
            .expect("Error reading the password");

        let key_hash = Sha256::digest(user_key.as_bytes());
        let key = Key::<Aes256Gcm>::from_slice(&key_hash);

        let cipher = Aes256Gcm::new(key);
        let data = read(&self.file_path)?;

        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let encrypted_data = cipher
            .encrypt(nonce, data.as_ref())
            .expect("Encryption error");

        let encrypted_file_path = format!("{}.aes", &self.file_path);

        FileUtils::create_path(&encrypted_file_path);

        let mut output = vec![];
        output.extend_from_slice(nonce);
        output.extend_from_slice(&encrypted_data);
        write(&encrypted_file_path, output)?;
        
        remove_file(&self.file_path)?;
        SuccessAlerts::dump(&encrypted_file_path);
        Ok(())
    }

    pub fn decrypt_and_read(&self) -> Result<Vec<u8>> { 
        let user_key = prompt_password("Enter the key (password): ")
            .expect("Error reading the password");
    
        let key_hash = Sha256::digest(user_key.as_bytes());
        let key = Key::<Aes256Gcm>::from_slice(&key_hash);
    
        let data = read(&self.file_path)?;
        let (nonce_bytes, encrypted_data) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let cipher = Aes256Gcm::new(key);
    
        let decrypted_data = cipher
            .decrypt(nonce, encrypted_data) 
            .expect("Decryption error");
    
        Ok(decrypted_data)
    }    

}
