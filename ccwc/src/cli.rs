use clap::{Arg, ArgAction, Command};

pub struct Cli {
    files: Vec<String>,
    count_bytes: bool,
    count_lines: bool,
    count_characters: bool,
    count_words: bool,
}

pub fn parse_command(cmd: String) -> Cli {
    let command = build_command();
    let matches = command.get_matches_from(cmd.split(" "));
    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect();
    let count_lines = matches.get_flag("l");
    let count_words = matches.get_flag("w");
    let mut count_bytes = matches.get_flag("c");
    let mut count_characters = matches.get_flag("m");
    if count_bytes && count_characters {
        let index_of_c = matches.index_of("c").unwrap();
        let index_of_m = matches.index_of("m").unwrap();
        count_bytes = index_of_c > index_of_m;
        count_characters = index_of_m > index_of_c;
    }
    if [count_lines, count_words, count_bytes, count_characters]
        .iter()
        .all(|x| !x)
    {
        return Cli {
            files,
            count_bytes: true,
            count_lines: true,
            count_characters: false,
            count_words: true,
        };
    }

    Cli {
        files,
        count_bytes,
        count_lines,
        count_characters,
        count_words,
    }
}

fn build_command() -> Command {
    Command::new("ccwc")
        .about("Coding Challenges - WC Tool")
        .author("Alfian Akmal Hanantio")
        .version("1.0.0")
        .arg(
            Arg::new("c")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("The number of bytes in each input file is written to the standard output.")
                .required(false),
        )
        .arg(
            Arg::new("l")
            .short('l')
                .action(ArgAction::SetTrue)
                .help("The number of lines in each input file is written to the standard output.")
                .required(false),
        )
        .arg(
            Arg::new("m")
            .short('m')
                .action(ArgAction::SetTrue)
                .help("The number of characters in each input file is written to the standard output. If the current locale does not support multibyte characters, this is equivalent to the -c option.")
                .required(false),
        )
        .arg(
            Arg::new("w")
            .short('w')
                .action(ArgAction::SetTrue)
                .help("The number of words in each input file is written to the standard output.")
                .required(false),
        ).arg(Arg::new("files").required(false).action(ArgAction::Append))
}

#[cfg(test)]
mod tests {

    use super::parse_command;

    #[test]
    fn given_no_flags_and_args_when_parse_command_then_returns_default_flag_with_empty_files() {
        // When
        let result = parse_command("ccwc".to_string());

        // Assert
        assert!(result.files.is_empty());
        assert!(result.count_bytes);
        assert!(result.count_lines);
        assert!(result.count_words);
        assert!(!result.count_characters)
    }

    #[test]
    fn given_multiple_args_when_parse_command_then_returns_multiple_files() {
        // When
        let result = parse_command("ccwc files1.txt files2.txt".to_string());

        // Assert
        assert_eq!(
            result.files,
            vec!["files1.txt".to_string(), "files2.txt".to_string()]
        );
        assert!(result.count_bytes);
        assert!(result.count_lines);
        assert!(result.count_words);
        assert!(!result.count_characters)
    }

    #[test]
    fn given_one_flag_single_args_when_parse_command_then_returns_multiple_files() {
        // When
        let result = parse_command("ccwc -l file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.count_bytes);
        assert!(result.count_lines);
        assert!(!result.count_words);
        assert!(!result.count_characters);

        // When
        let result = parse_command("ccwc -c file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(result.count_bytes);
        assert!(!result.count_lines);
        assert!(!result.count_words);
        assert!(!result.count_characters);

        // When
        let result = parse_command("ccwc -w file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.count_bytes);
        assert!(!result.count_lines);
        assert!(result.count_words);
        assert!(!result.count_characters);

        // When
        let result = parse_command("ccwc -m file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.count_bytes);
        assert!(!result.count_lines);
        assert!(!result.count_words);
        assert!(result.count_characters)
    }

    #[test]
    fn given_flags_c_and_m_when_parse_command_then_override_previous_flag() {
        // When
        let result = parse_command("ccwc -c -m file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.count_bytes);
        assert!(!result.count_lines);
        assert!(!result.count_words);
        assert!(result.count_characters);

        // When
        let result = parse_command("ccwc -cm file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.count_bytes);
        assert!(!result.count_lines);
        assert!(!result.count_words);
        assert!(result.count_characters);

        let result = parse_command("ccwc -m -c file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(result.count_bytes);
        assert!(!result.count_lines);
        assert!(!result.count_words);
        assert!(!result.count_characters);
    }
}
