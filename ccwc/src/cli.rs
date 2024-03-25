use clap::Parser;

#[derive(Parser, Debug)]
#[command(version("1.0"), about("Coding Challenges - WC Tool"), long_about = None)]
pub struct Cli {
    /// Files to input
    files: Vec<String>,

    /// The number of bytes in each input file is written to the standard output.
    #[arg(short('c'), action = clap::ArgAction::SetTrue)]
    count_bytes: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short('l'), action = clap::ArgAction::SetTrue)]
    count_lines: bool,

    /// The number of characters in each input file is written to the standard output. If the current locale does not support multibyte characters, this is equivalent to the -c option.
    #[arg(short('m'), action = clap::ArgAction::SetTrue)]
    count_characters: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short('w'), action = clap::ArgAction::SetTrue)]
    count_words: bool,
}

pub fn parse_command(cmd: String) -> Cli {
    let cli = Cli::parse_from(cmd.split(" "));
    match (
        cli.count_bytes,
        cli.count_characters,
        cli.count_lines,
        cli.count_words,
    ) {
        (false, false, false, false) => Cli {
            files: cli.files,
            count_bytes: true,
            count_lines: true,
            count_characters: false,
            count_words: true,
        },
        _ => cli,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_command, Cli};

    #[test]
    fn test_parse_command() {
        let test_cases = vec![
            (
                "ccwc",
                Cli {
                    files: vec![],
                    count_bytes: true,
                    count_characters: false,
                    count_lines: true,
                    count_words: true,
                },
            ),
            (
                "ccwc -c test.txt",
                Cli {
                    files: vec!["test.txt".to_string()],
                    count_bytes: true,
                    count_characters: false,
                    count_lines: false,
                    count_words: false,
                },
            ),
            (
                "ccwc -cw test.txt test_01.txt",
                Cli {
                    files: vec!["test.txt".to_string(), "test_01.txt".to_string()],
                    count_bytes: true,
                    count_characters: false,
                    count_lines: false,
                    count_words: true,
                },
            ),
            (
                "ccwc -c -m test.txt",
                Cli {
                    files: vec!["test.txt".to_string()],
                    count_bytes: true,
                    count_characters: true,
                    count_lines: false,
                    count_words: false,
                },
            ),
            (
                "ccwc -c -l test.txt",
                Cli {
                    files: vec!["test.txt".to_string()],
                    count_bytes: true,
                    count_characters: false,
                    count_lines: true,
                    count_words: false,
                },
            ),
        ];
        test_cases.iter().for_each(|tc| {
            let result = parse_command(tc.0.to_string());
            assert_eq!(
                result.count_bytes, tc.1.count_bytes,
                "Failed command: {}",
                tc.0
            );
            assert_eq!(
                result.count_characters, tc.1.count_characters,
                "Failed command: {}",
                tc.0
            );
            assert_eq!(
                result.count_lines, tc.1.count_lines,
                "Failed command: {}",
                tc.0
            );
            assert_eq!(
                result.count_words, tc.1.count_words,
                "Failed command: {}",
                tc.0
            );
            assert_eq!(result.files, tc.1.files, "Failed command: {}", tc.0)
        });
    }
}
