use crate::{
    helpers::env::Env,
    constants::api::api_names::ApiNames,
};

pub struct APIToken;

impl APIToken {

    pub fn value(&self) -> String {
        let api_token = Env.system(ApiNames::Env.as_str());
        format!("Bearer {}", api_token)
    }

}
