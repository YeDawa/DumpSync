pub enum SyntaxSkip {
    SkipLine,
    SkipTables,
}

impl SyntaxSkip {

    pub fn as_str(&self) -> &'static str {
        match self {
            SyntaxSkip::SkipLine => "-- skip line",
            SyntaxSkip::SkipTables => "-- skip tables",
        }
    }

}