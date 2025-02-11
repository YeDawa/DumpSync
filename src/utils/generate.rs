use std::time::{
    SystemTime, 
    UNIX_EPOCH
};

pub struct Generate;

impl Generate {

    fn get_random_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
    
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let mut state = seed;
        for i in 0..16 {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            bytes[i] = (state.wrapping_mul(2685821657736338717) >> 59) as u8;
        }
    
        bytes
    }
    
    pub fn uuid_v4(&self) -> String {
        let mut uuid = self.get_random_bytes();

        uuid[6] = (uuid[6] & 0x0F) | 0x40;
        uuid[8] = (uuid[8] & 0x3F) | 0x80;

        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            u32::from_be_bytes([uuid[0], uuid[1], uuid[2], uuid[3]]),
            u16::from_be_bytes([uuid[4], uuid[5]]),
            u16::from_be_bytes([uuid[6], uuid[7]]),
            u16::from_be_bytes([uuid[8], uuid[9]]),
            u64::from_be_bytes([
                uuid[10], uuid[11], uuid[12], uuid[13], uuid[14], uuid[15], 0, 0
            ]) >> 16
        )
    }

    pub fn random_string(&self, size: usize) -> String {
        let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut result = String::new();

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;

        let mut rng = current_time;

        for _ in 0..size {
            let idx = rng % charset.len();
            result.push(charset[idx] as char);
            rng = rng >> 1;
        }

        result
    }

}
