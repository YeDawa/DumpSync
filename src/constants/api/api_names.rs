pub enum ApiNames {
    Env,
}

impl ApiNames {

    pub fn as_str(&self) -> &'static str {
        match self {
            ApiNames::Env => "DS_API_KEY",
        }
    }

}