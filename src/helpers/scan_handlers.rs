use reqwest;
use regex::Regex;

use std::{
    fs::File,
    path::Path,
    error::Error,

    io::{
        self, 
        BufRead
    },
};

pub struct ScanHandlers;

impl ScanHandlers {    

    pub fn load_patterns_from_file(&self, path: &str) -> Result<Vec<Regex>, Box<dyn Error>> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut patterns = Vec::new();
        for line in reader.lines() {
            let line = line?;

            if !line.trim().is_empty() {
                match Regex::new(&line) {
                    Ok(regex) => patterns.push(regex),
                    Err(e) => eprintln!("Invalid regex '{}': {}", line, e),
                }
            }
        }

        Ok(patterns)
    }

    pub async fn load_patterns_from_url(&self, url: &str) -> Result<Vec<Regex>, Box<dyn Error>> {
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(format!("Erro ao acessar a URL: {}", url).into());
        }

        let body = response.text().await?;
        let mut patterns = Vec::new();
        for line in body.lines() {
            if !line.trim().is_empty() {
                match Regex::new(line) {
                    Ok(regex) => patterns.push(regex),
                    Err(e) => eprintln!("Regex inválida '{}': {}", line, e),
                }
            }
        }

        Ok(patterns)
    }

    pub fn is_potential_xss(&self, value: &str, patterns: &[Regex]) -> bool {
        for pattern in patterns {
            if pattern.is_match(value) {
                return true;
            }
        }

        false
    }
    
}
