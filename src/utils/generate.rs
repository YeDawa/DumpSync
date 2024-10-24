use rand::{
    Rng,
    distributions::Alphanumeric,
};

pub struct Generate;

impl Generate {

    pub fn random_string(&self, size: usize) -> String {
        let mut rng = rand::thread_rng();
        
        (0..size)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect()
    }

}
