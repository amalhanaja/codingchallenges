pub fn format_output(counts: Vec<usize>, name: Option<String>, digit: usize) -> String {
    let counts_str = counts
        .into_iter()
        .map(|n| format!("{:>digit$}", n, digit = digit))
        .collect::<Vec<_>>()
        .join(" ");
    format!("{} {}", counts_str, name.unwrap_or_default())
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::formatter::format_output;

    #[test]
    fn test_format_output() {
        assert_eq!(
            "1 2 3 test.txt",
            format_output(vec![1, 2, 3], Some("test.txt".to_string()), 1)
        );
        assert_eq!("1 2 3", format_output(vec![1, 2, 3], None, 1));
        assert_eq!("   1  200 3000", format_output(vec![1, 200, 3000], None, 4))
    }
}
