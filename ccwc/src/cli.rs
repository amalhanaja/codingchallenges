use std::{
    error::Error,
    fs,
    io::{self, BufReader, Read},
    ops::Deref,
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
    let options = match (count_lines, count_words, count_bytes, count_characters) {
        (false, false, false, false) => CountOptions::new(true, true, false, true),
        _ => CountOptions::new(count_bytes, count_lines, count_characters, count_words),
    };

    Cli { files, options }
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
    fn get_content(&self, reader: &mut dyn io::BufRead) -> Vec<Result<String, Box<dyn Error>>> {
        if self.files.is_empty() {
            return self.get_content_from_input(reader);
        }
        self.get_content_from_files()
    }

    fn get_content_from_files(&self) -> Vec<Result<String, Box<dyn Error>>> {
        (*self.files)
            .into_iter()
            .map(|file| fs::read_to_string(file).map_err(Box::from))
            .collect()
    }

    fn get_content_from_input(
        &self,
        reader: &mut dyn io::BufRead,
    ) -> Vec<Result<String, Box<dyn Error>>> {
        let mut content = String::new();
        let mut reader = BufReader::new(reader);
        match reader.read_to_string(&mut content) {
            Ok(_) => vec![Ok(content)],
            Err(err) => vec![Err(Box::new(err))],
        }
    }

    pub fn execute(&self, reader: &mut dyn io::BufRead) -> String {
        let contents = self.get_content(reader);
        let result = contents
            .into_iter()
            .map(|c| c.map(|content| count(&*content, &self.options)))
            .collect::<Vec<Result<Vec<usize>, Box<dyn Error>>>>();
        let summary = (*result)
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.as_ref().unwrap().deref())
            .fold(Vec::<usize>::new(), |acc, current| {
                current
                    .into_iter()
                    .enumerate()
                    .map(|(i, current_value)| (current_value + acc.get(i).unwrap_or(&0)))
                    .collect()
            });
        let max_digit = (*summary).into_iter().max().unwrap_or(&0).to_string().len();
        let counts = result
            .deref()
            .into_iter()
            .enumerate()
            .map(|(i, res)| match res {
                Ok(counts) => format_output(
                    counts.deref().to_vec(),
                    self.files.get(i).cloned(),
                    max_digit,
                ),
                Err(err) => format!("wc: {}", err),
            })
            .collect::<Vec<_>>()
            .join("\n");
        if result.len() <= 1 {
            return counts;
        }
        [
            counts,
            format_output(summary, Some("total".to_string()), max_digit),
        ]
        .join("\n")
    }
}

#[cfg(test)]
mod tests {

    use std::{
        io::{self, BufRead, Error},
        vec,
    };

    use super::parse_command;

    #[test]
    fn given_no_flags_and_args_when_parse_command_then_returns_default_flag_with_empty_files() {
        // Act
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
        // Act
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
        // Act
        let result = parse_command("ccwc -l file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(!result.options.count_characters);

        // Act
        let result = parse_command("ccwc -c file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(!result.options.count_characters);

        // Act
        let result = parse_command("ccwc -w file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(result.options.count_words);
        assert!(!result.options.count_characters);

        // Act
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
        // Act
        let result = parse_command("ccwc -c -m file.txt".to_string());

        // Assert
        assert_eq!(result.files, vec!["file.txt".to_string()]);
        assert!(!result.options.count_bytes);
        assert!(!result.options.count_lines);
        assert!(!result.options.count_words);
        assert!(result.options.count_characters);

        // Act
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

    #[test]
    fn test_execute_single_file() {
        // Act
        let result =
            parse_command("ccwc -c -m test.txt".to_string()).execute(&mut io::stdin().lock());

        // Assert
        assert_eq!("339292 test.txt".to_string(), result)
    }

    #[test]
    fn test_execute_multiple_file() {
        // Act
        let result = parse_command("ccwc -c -m test.txt test.txt".to_string())
            .execute(&mut io::stdin().lock());

        // Assert
        assert_eq!(
            "339292 test.txt\n339292 test.txt\n678584 total".to_string(),
            result,
        );
    }

    #[test]
    fn test_execute_multiple_file_with_file_not_found() {
        // Act
        let result = parse_command("ccwc -c -m test.txt test.txt not_found".to_string())
            .execute(&mut io::stdin().lock());

        // Assert
        assert_eq!(
            "339292 test.txt\n339292 test.txt\nwc: No such file or directory (os error 2)\n678584 total".to_string(),
            result,
        );
    }

    #[test]
    fn test_execute_from_reader() {
        let result = parse_command("ccwc -c -m".to_string()).execute(&mut "Alfian".as_bytes());

        // Assert
        assert_eq!("6".to_string(), result,);
    }

    #[test]
    fn test_execute_from_error_reader() {
        struct FakeReader;
        impl io::Read for FakeReader {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(Error::new(io::ErrorKind::Other, "failed"))
            }
        }
        impl io::BufRead for FakeReader {
            fn fill_buf(&mut self) -> io::Result<&[u8]> {
                unreachable!()
            }

            fn consume(&mut self, _: usize) {}
        }
        let reader = &mut FakeReader {};
        let result = parse_command("ccwc -c -m".to_string()).execute(reader);

        // Assert
        assert_eq!("wc: failed".to_string(), result);
    }
}
