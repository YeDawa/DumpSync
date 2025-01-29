use rand::Rng;

pub struct Generate;

impl Generate {

    pub fn random_string(&self, size: usize) -> String {
        let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();
        
        (0..size)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }

}
