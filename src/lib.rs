use chrono::{DateTime, Utc};
use std::fs::read_dir;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::prelude::MetadataExt;
use std::time::SystemTime;
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug, Clone, PartialEq)]
enum CLIOptions {
    All,
    List,
}

#[derive(Debug, Clone)]
pub struct Config {
    options: Vec<CLIOptions>,
    path: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        if args.len() == 1 {
            return Config {
                options: vec![],
                path: ".".to_string(),
            };
        }

        let options = Config::parse_options(&args);

        if args.len() == 2 {
            return Config {
                options: options,
                path: ".".to_string(),
            };
        }

        let path = if options.is_empty() {
            args[1].clone()
        } else {
            args[2].clone()
        };

        Config {
            options: options,
            path: path,
        }
    }

    fn parse_path(args: &Vec<String>) -> String {
        unimplemented!()
    }

    fn parse_options(args: &Vec<String>) -> Vec<CLIOptions> {
        let options_string: String = args
            .iter()
            .filter(|a| a.starts_with('-'))
            .map(|a| a[1..].to_string())
            .collect::<Vec<String>>()
            .join("");

        let mut result = vec![];
        for c in options_string.chars() {
            match c {
                'a' => result.push(CLIOptions::All),
                'l' => result.push(CLIOptions::List),
                _ => {}
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
struct Entry {
    permissions: u32,
    owner: String,
    group: String,
    file_size: u64,
    updated_at: String,
    filename: String,
    file_type: EntryType,
}

#[derive(Debug, Clone, PartialEq)]
enum EntryType {
    Hidden,
    Normal,
}

#[derive(Debug, Clone)]
pub struct CLI {
    entries: Vec<Entry>,
    config: Config,
}

impl CLI {
    pub fn from_config(config: Config) -> Self {
        let mut results: Vec<Entry> = vec![];

        if let Ok(entries) = read_dir(&config.path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let permissions = entry.metadata().unwrap().permissions().mode();
                    let owner = get_user_by_uid(entry.metadata().unwrap().uid())
                        .unwrap()
                        .name()
                        .to_string_lossy()
                        .to_string();
                    let group = get_group_by_gid(entry.metadata().unwrap().gid())
                        .unwrap()
                        .name()
                        .to_string_lossy()
                        .to_string();
                    let system_time: SystemTime = entry.metadata().unwrap().modified().unwrap();
                    let datetime: DateTime<Utc> = system_time.into();
                    let updated_at = datetime.format("%b %d %H:%M").to_string();
                    let file_size = entry.metadata().unwrap().len();
                    let filename = entry.file_name().to_str().unwrap().to_string();
                    let file_type = if filename.starts_with('.') {
                        EntryType::Hidden
                    } else {
                        EntryType::Normal
                    };
                    results.push(Entry {
                        permissions,
                        owner,
                        group,
                        file_size,
                        updated_at,
                        filename,
                        file_type,
                    })
                }
            }
        }

        results.sort_by_key(|entry| entry.filename.clone());

        CLI {
            entries: results,
            config: config,
        }
    }

    pub fn run(&self) {
        let entries = if !self.config.options.contains(&CLIOptions::All) {
            self.entries
                .iter()
                .filter(|e| e.file_type == EntryType::Normal)
                .map(|e| e.clone())
                .collect()
        } else {
            self.entries.clone()
        };

        if self.config.options.contains(&CLIOptions::List) {
            for item in entries {
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    item.permissions,
                    item.owner,
                    item.group,
                    item.file_size,
                    item.updated_at,
                    item.filename,
                )
            }
        } else {
            for item in entries {
                print!("{} ", item.filename,)
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_options_returns_valid_vec() {
        let test_cases: Vec<Vec<String>> = vec![
            vec!["bin".to_string()],
            vec!["bin".to_string(), "-l".to_string()],
            vec!["bin".to_string(), "-a".to_string()],
            vec!["bin".to_string(), "-la".to_string()],
            vec!["bin".to_string(), "-l".to_string(), "-a".to_string()],
            vec!["bin".to_string(), "-a".to_string(), "-l".to_string()],
            vec!["bin".to_string(), ".".to_string()],
            vec!["bin".to_string(), "-l".to_string(), ".".to_string()],
            vec!["bin".to_string(), "-a".to_string(), ".".to_string()],
            vec!["bin".to_string(), "-la".to_string(), ".".to_string()],
            vec![
                "bin".to_string(),
                "-l".to_string(),
                "-a".to_string(),
                ".".to_string(),
            ],
            vec![
                "bin".to_string(),
                "-a".to_string(),
                "-l".to_string(),
                ".".to_string(),
            ],
        ];

        let expected: Vec<Vec<CLIOptions>> = vec![
            vec![],
            vec![CLIOptions::List],
            vec![CLIOptions::All],
            vec![CLIOptions::List, CLIOptions::All],
            vec![CLIOptions::List, CLIOptions::All],
            vec![CLIOptions::All, CLIOptions::List],
            vec![],
            vec![CLIOptions::List],
            vec![CLIOptions::All],
            vec![CLIOptions::List, CLIOptions::All],
            vec![CLIOptions::List, CLIOptions::All],
            vec![CLIOptions::All, CLIOptions::List],
        ];

        let zipped: Vec<(&Vec<String>, Vec<CLIOptions>)> =
            test_cases.iter().zip(expected).collect();

        for (test_args, expect) in zipped {
            let actual = Config::parse_options(test_args);
            assert_eq!(expect, actual, "input args: {:?}", test_args);
        }
    }

    #[test]
    fn parse_path_returns_current_dir_when_no_path_is_specified() {
        let test_cases: Vec<Vec<String>> = vec![
            vec!["bin".to_string()],
            vec!["bin".to_string(), "-l".to_string()],
            vec!["bin".to_string(), "-la".to_string()],
            vec!["bin".to_string(), "-l".to_string(), "-a".to_string()],
        ];

        let expected: Vec<String> = vec![
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
        ];

        let zipped: Vec<(&Vec<String>, String)> =
            test_cases.iter().zip(expected).collect();

        for (test_args, expect) in zipped {
            let actual = Config::parse_path(test_args);
            assert_eq!(expect, actual, "input args: {:?}", test_args);
        }
    }

    #[test]
    fn parse_path_returns_path_arg() {
        let test_cases: Vec<Vec<String>> = vec![
            vec!["bin".to_string(), "./relative/path".to_string()],
            vec!["bin".to_string(), "-l".to_string(), "/some/path".to_string()],
            vec!["bin".to_string(), "-la".to_string(), "/some/other/path".to_string()],
            vec!["bin".to_string(), "-l".to_string(), "-a".to_string(), "./relative/path".to_string()],
        ];

        let expected: Vec<String> = vec![
            "./relative/path".to_string(),
            "/some/path".to_string(),
            "/some/other/path".to_string(),
            "./relative/path".to_string(),
        ];

        let zipped: Vec<(&Vec<String>, String)> =
            test_cases.iter().zip(expected).collect();

        for (test_args, expect) in zipped {
            let actual = Config::parse_path(test_args);
            assert_eq!(expect, actual, "input args: {:?}", test_args);
        }
    }
}
