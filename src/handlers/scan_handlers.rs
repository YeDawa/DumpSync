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

use crate::constants::urls::*;

pub struct ScanHandlers;

impl ScanHandlers {

    pub fn load_patterns_from_file(&self, path: &str) -> Result<Vec<Regex>, Box<dyn Error>> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut patterns = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            let pattern = line.split("//").next().unwrap_or("").trim();

            if !pattern.is_empty() {
                match Regex::new(pattern) {
                    Ok(regex) => patterns.push(regex),
                    Err(e) => eprintln!("Invalid regex '{}': {}", pattern, e),
                }
            }
        }

        Ok(patterns)
    }

    pub async fn load_patterns_from_url(&self, url: &str) -> Result<Vec<Regex>, Box<dyn Error>> {
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(
                format!("Error accessing URL: {}", url).into()
            );
        }

        let body = response.text().await?;
        let mut patterns = Vec::new();
        for line in body.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            let pattern = trimmed.split("//").next().unwrap().trim();

            if !pattern.is_empty() {
                match Regex::new(pattern) {
                    Ok(regex) => patterns.push(regex),
                    Err(e) => eprintln!("Invalid regex '{}': {}", pattern, e),
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

    pub async fn read_patterns(&self, payload: Option<String>) -> Result<Vec<Regex>, Box<dyn Error>> {
        let patterns = match payload {
            Some(value) => {
                if value.starts_with("http://") || value.starts_with("https://") {
                    ScanHandlers.load_patterns_from_url(&value).await?
                } else if Path::new(&value).exists() {
                    ScanHandlers.load_patterns_from_file(&value)?
                } else {
                    return Err("Invalid payload source, not a valid file or URL.".into());
                }
            }

            None => {
                ScanHandlers.load_patterns_from_url(Urls::as_str(UrlsNames::XssDetectRegex)).await?
            }
        };

        Ok(patterns)
    }
    
}
