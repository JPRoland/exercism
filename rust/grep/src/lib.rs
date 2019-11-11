use failure::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    list_files: bool,
    case_insensitive: bool,
    inverted: bool,
    full_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            line_numbers: flags.contains(&"-n"),
            list_files: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            inverted: flags.contains(&"-v"),
            full_match: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matches = Vec::new();
    for file in files {
        let f = File::open(file)?;
        for (n, line) in BufReader::new(f).lines().enumerate() {
            if let Ok(mut text) = line {
                let text_match = match (flags.full_match, flags.case_insensitive) {
                    (true, true) => text.to_lowercase() == pattern.to_lowercase(),
                    (true, false) => text == pattern,
                    (false, true) => text
                        .to_lowercase()
                        .contains(pattern.to_lowercase().as_str()),
                    (false, false) => text.contains(pattern),
                };

                if flags.line_numbers {
                    text = format!("{}:{}", n + 1, text);
                }
                if files.len() > 1 {
                    text = format!("{}:{}", file, text);
                }
                if (text_match && !flags.inverted) || (!text_match && flags.inverted) {
                    if flags.list_files {
                        matches.push(file.to_string());
                        break;
                    } else {
                        matches.push(text);
                    }
                }
            }
        }
    }
    Ok(matches)
}
