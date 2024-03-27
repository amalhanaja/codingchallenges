pub fn count_bytes(content: &str) -> usize {
    content.bytes().count()
}

pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}

pub fn count_characters(content: &str) -> usize {
    content.chars().count()
}

pub fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use crate::counter::{count_bytes, count_characters, count_lines, count_words};

    #[test]
    fn test_count_bytes() {
        assert_eq!(count_bytes("Alfian"), 6);
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("Singel line"), 1);
        assert_eq!(count_lines("Multiple\nlines"), 2);
    }

    #[test]
    fn test_count_characters() {
        assert_eq!(count_characters("Alfian"), 6);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Single line"), 2);
    }
}
