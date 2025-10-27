use crate::{
    helpers::env::Env,
    constants::urls::*,
    constants::api::api_names::ApiNames,
};

pub struct APIInit;

impl APIInit {

    pub fn token_value() -> String {
        let api_token = Env.system(ApiNames::Env.as_str());
        format!("Bearer {}", api_token)
    }

    pub fn url_builder(endpoint: &str) -> String {
        let mut api_url = String::from(Urls::as_str(UrlsNames::DumpsyncApi));
        api_url.push_str("backups/");
        api_url.push_str(endpoint);
        api_url
    }

}
