pub enum Protocols {
    Http,
    Https,
}

impl Protocols {

    pub fn as_str(&self) -> &'static str {
        match self {
            Protocols::Http => "http://",
            Protocols::Https => "https://",
        }
    }

}