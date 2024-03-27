use std::{
    error::Error,
    fs,
    io::{self, BufReader, Read},
    vec,
};

use clap::{Arg, ArgAction, Command};

use crate::{counter::count, formatter::format_output, options::CountOptions};

pub struct Cli {
    files: Vec<String>,
    options: CountOptions,
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
            options: CountOptions::new(true, true, false, true),
        };
    }

    Cli {
        files,
        options: CountOptions::new(count_bytes, count_lines, count_characters, count_words),
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

impl Cli {
    fn get_content(&self) -> Vec<Result<String, Box<dyn Error>>> {
        if self.files.is_empty() {
            return self.get_content_from_input();
        }
        self.get_content_from_files()
    }

    fn get_content_from_files(&self) -> Vec<Result<String, Box<dyn Error>>> {
        (*self.files)
            .into_iter()
            .map(|file| fs::read_to_string(file).map_err(Box::from))
            .collect()
    }

    fn get_content_from_input(&self) -> Vec<Result<String, Box<dyn Error>>> {
        let mut content = String::new();
        let mut reader = BufReader::new(io::stdin());
        match reader.read_to_string(&mut content) {
            Ok(_) => vec![Ok(content)],
            Err(err) => vec![Err(Box::new(err))],
        }
    }

    pub fn execute(&self) -> String {
        let contents = self.get_content();
        let result = contents
            .into_iter()
            .map(|c| c.map(|content| count(&*content, &self.options)))
            .collect::<Vec<Result<Vec<usize>, Box<dyn Error>>>>();
        let max_digit = (*result).into_iter().fold(0usize, |acc, current| {
            let max = current
                .as_ref()
                .map(|counts| counts.into_iter().max().unwrap_or(&0).to_string().len())
                .unwrap_or(0);
            acc.max(max)
        });
        result
            .into_iter()
            .enumerate()
            .map(|(i, res)| match res {
                Ok(counts) => format_output(counts, self.files.get(i).cloned(), max_digit),
                Err(err) => format!("wc: {}", err),
            })
            .collect::<Vec<_>>()
            .join("")
    }
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
        assert!(result.options.count_bytes);
        assert!(result.options.count_lines);
        assert!(result.options.count_words);
        assert!(!result.options.count_characters)
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
        assert!(result.options.count_bytes);
        assert!(result.options.count_lines);
        assert!(result.options.count_words);
        assert!(!result.options.count_characters)
    }

    #[test]
    fn given_one_flag_single_args_when_parse_command_then_returns_multiple_files() {
        // When
        let result = parse_command("ccwc -l file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(!result.options.count_characters);

        // When
        let result = parse_command("ccwc -c file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(!result.options.count_characters);

        // When
        let result = parse_command("ccwc -w file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(result.options.count_words);
        assert!(!result.options.count_characters);

        // When
        let result = parse_command("ccwc -m file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(result.options.count_characters)
    }

    #[test]
    fn given_flags_c_and_m_when_parse_command_then_override_previous_flag() {
        // When
        let result = parse_command("ccwc -c -m file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(result.options.count_characters);

        // When
        let result = parse_command("ccwc -cm file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(result.options.count_characters);

        let result = parse_command("ccwc -m -c file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(!result.options.count_characters);
    }
}
