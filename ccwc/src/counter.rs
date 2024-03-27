use crate::options;

fn count_bytes(content: &str) -> usize {
    content.bytes().count()
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}

fn count_characters(content: &str) -> usize {
    content.chars().count()
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn count(content: &str, options: &options::CountOptions) -> Vec<usize> {
    let mut vec = Vec::<usize>::new();
    if options.count_lines {
        vec.push(count_lines(content));
    }
    if options.count_words {
        vec.push(count_words(content));
    }
    if options.count_bytes {
        vec.push(count_bytes(content));
    }
    if options.count_characters {
        vec.push(count_characters(content));
    }
    vec
}

#[cfg(test)]
mod tests {
    use crate::{
        counter::{count, count_bytes, count_characters, count_lines, count_words},
        options::CountOptions,
    };

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

    #[test]
    fn test_count() {
        assert_eq!(
            count(
                "Multiple\nlines count",
                &CountOptions::new(true, true, true, true)
            ),
            vec![2, 3, 20, 20]
        );
        assert_eq!(
            count(
                "Multiple\nlines count",
                &CountOptions::new(false, true, true, true)
            ),
            vec![2, 3, 20]
        );
        assert_eq!(
            count(
                "Multiple\nlines count",
                &CountOptions::new(false, false, true, true)
            ),
            vec![3, 20]
        );
        assert_eq!(
            count(
                "Multiple\nlines count",
                &CountOptions::new(false, false, false, true)
            ),
            vec![3]
        );
        assert_eq!(
            count(
                "Multiple\nlines count",
                &CountOptions::new(false, false, false, false)
            ),
            vec![]
        );
    }
}
