use std::{
    fs::File,
    collections::HashMap,

    io::{
        Read,
        Result, 
    },
};

pub struct Entropy<'a> {
    file_path: &'a str,
}

impl<'a> Entropy<'a> {

    pub fn new(file_path: &'a str) -> Self {
        Self {
            file_path
        }
    }

    pub fn calculate(&self) -> Result<f64> {
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
            .sum()
        )
    }

}
