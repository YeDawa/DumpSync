pub enum ApiNames {
    Env,
}

pub struct APIInit;

impl APIInit {

    pub fn as_str(api: ApiNames) -> &'static str {
        match api {
            ApiNames::Env => "DS_API_KEY",
        }
    }

}