use std::time::{
    SystemTime, 
    UNIX_EPOCH
};

pub struct Generate;

impl Generate {

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
