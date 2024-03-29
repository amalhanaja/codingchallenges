pub struct CountOptions {
    pub count_bytes: bool,
    pub count_lines: bool,
    pub count_characters: bool,
    pub count_words: bool,
}

impl CountOptions {
    pub fn new(
        count_bytes: bool,
        count_lines: bool,
        count_characters: bool,
        count_words: bool,
    ) -> Self {
        Self {
            count_bytes,
            count_lines,
            count_characters,
            count_words,
        }
    }
}
