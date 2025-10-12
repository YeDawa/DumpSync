pub enum SyntaxSkip {
    SkipLine,
}

impl SyntaxSkip {

    pub fn as_str(&self) -> &'static str {
        match self {
            SyntaxSkip::SkipLine => "-- skip line",
        }
    }

}