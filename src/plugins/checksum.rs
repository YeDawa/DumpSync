use crc32fast::Hasher as Crc32Hasher;

use md5::{
    Md5, 
    Digest as Md5Digest
};

use sha1::Sha1;
use sha2::Sha256;

use std::{
    fs::{
        File, 
        OpenOptions,
    },

    io::{
        self, 
        Read, 
        Write
    },
};

use crate::ui::success_alerts::SuccessAlerts;

pub struct Checksum {
    file_path: String,
    output_path: String,
}

impl Checksum {

    pub fn new(file_path: &str, output_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
        }
    }

    pub fn calculate_hashes(&self) -> io::Result<(u32, String, String, String)> {
        let mut file = File::open(&self.file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
    
        let mut crc32_hasher = Crc32Hasher::new();
        crc32_hasher.update(&buffer);
        let crc32 = crc32_hasher.finalize();
    
        let mut md5_hasher = Md5::new();
        md5_hasher.update(&buffer);
        let md5 = format!("{:x}", md5_hasher.finalize());
    
        let mut sha1_hasher = Sha1::new();
        sha1_hasher.update(&buffer);
        let sha1 = format!("{:x}", sha1_hasher.finalize());
    
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&buffer);
        let sha256 = format!("{:x}", sha256_hasher.finalize());
    
        Ok((crc32, md5, sha1, sha256))
    }

    pub fn generated(&self) -> io::Result<()> {
        let (crc32, md5, sha1, sha256) = &self.calculate_hashes()?;

        let mut output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.output_path)?;
    
        writeln!(output_file, "CRC32: {:08x}", crc32)?;
        writeln!(output_file, "MD5: {}", md5)?;
        writeln!(output_file, "SHA1: {}", sha1)?;
        writeln!(output_file, "SHA256: {}", sha256)?;
    
        SuccessAlerts::checksum(&self.output_path);
        Ok(())
    }

}