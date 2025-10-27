use crate::constants::urls::*;

pub struct APIEndpoints;

impl APIEndpoints {

    pub fn backups(&self, endpoint: &str) -> String {
        let mut api_url = String::from(Urls::as_str(UrlsNames::DumpsyncApi));
        api_url.push_str("backups/");
        api_url.push_str(endpoint);
        api_url
    }

    pub fn login(&self) -> &str {
        Urls::as_str(UrlsNames::DumpsyncApiKey)
    }

}
