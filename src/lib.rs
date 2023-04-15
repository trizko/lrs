use chrono::{DateTime, Utc};
use std::env::Args;
use std::fs::{read_dir, DirEntry};
use std::io::{Error, ErrorKind};
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
        let options = Config::parse_options(&args);
        let path = Config::parse_path(&args);

        Config {
            options: options,
            path: path,
        }
    }

    fn parse_path(args: &Vec<String>) -> String {
        let no_bin_args = &args[1..];
        let no_flags_args: Vec<String> = no_bin_args
            .iter()
            .filter(|p| !p.starts_with('-'))
            .map(|p| p.clone())
            .collect();

        match no_flags_args.get(0) {
            Some(path) => path.clone(),
            None => ".".to_string(),
        }
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
    modified_at: String,
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
    pub fn from_config(args: Args) -> Result<Self, Error> {
        let config: Config = Config::new(args.collect());
        let mut results: Vec<Entry> = vec![];
        let entries: Vec<Result<DirEntry, _>> = read_dir(&config.path)?.into_iter().collect();

        for entry in entries {
            if let Ok(entry) = entry {
                results.push(Entry {
                    permissions: CLI::get_permissions(&entry)?,
                    owner: CLI::get_owner_name(&entry)?,
                    group: CLI::get_group_name(&entry)?,
                    file_size: CLI::get_file_size(&entry)?,
                    modified_at: CLI::get_modified_at(&entry)?,
                    filename: CLI::get_filename(&entry)?,
                    file_type: CLI::get_file_type(&CLI::get_filename(&entry)?),
                })
            }
        }

        results.sort_by_key(|entry| entry.filename.clone());

        Ok(CLI {
            entries: results,
            config: config,
        })
    }

    fn get_permissions(entry: &DirEntry) -> Result<u32, Error> {
        Ok(entry.metadata()?.permissions().mode())
    }

    fn get_owner_name(entry: &DirEntry) -> Result<String, Error> {
        Ok(get_user_by_uid(entry.metadata()?.uid())
            .ok_or(Error::new(ErrorKind::Other, "Error occured getting uid"))?
            .name()
            .to_string_lossy()
            .to_string())
    }

    fn get_group_name(entry: &DirEntry) -> Result<String, Error> {
        Ok(get_group_by_gid(entry.metadata()?.gid())
            .ok_or(Error::new(ErrorKind::Other, "Error occured getting uid"))?
            .name()
            .to_string_lossy()
            .to_string())
    }

    fn get_file_size(entry: &DirEntry) -> Result<u64, Error> {
        Ok(entry.metadata()?.len())
    }

    fn get_modified_at(entry: &DirEntry) -> Result<String, Error> {
        let system_time: SystemTime = entry.metadata()?.modified()?;
        let datetime: DateTime<Utc> = system_time.into();

        Ok(datetime.format("%b %d %H:%M").to_string())
    }

    fn get_filename(entry: &DirEntry) -> Result<String, Error> {
        Ok(entry
            .file_name()
            .to_str()
            .ok_or(Error::new(
                ErrorKind::Other,
                "filename does not contain valid unicode",
            ))?
            .to_string())
    }

    fn get_file_type(filename: &String) -> EntryType {
        if filename.starts_with('.') {
            EntryType::Hidden
        } else {
            EntryType::Normal
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
                    item.modified_at,
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

        let zipped: Vec<(&Vec<String>, String)> = test_cases.iter().zip(expected).collect();

        for (test_args, expect) in zipped {
            let actual = Config::parse_path(test_args);
            assert_eq!(expect, actual, "input args: {:?}", test_args);
        }
    }

    #[test]
    fn parse_path_returns_path_arg() {
        let test_cases: Vec<Vec<String>> = vec![
            vec!["bin".to_string(), "./relative/path".to_string()],
            vec![
                "bin".to_string(),
                "-l".to_string(),
                "/some/path".to_string(),
            ],
            vec![
                "bin".to_string(),
                "-la".to_string(),
                "/some/other/path".to_string(),
            ],
            vec![
                "bin".to_string(),
                "-l".to_string(),
                "-a".to_string(),
                "./relative/path".to_string(),
            ],
        ];

        let expected: Vec<String> = vec![
            "./relative/path".to_string(),
            "/some/path".to_string(),
            "/some/other/path".to_string(),
            "./relative/path".to_string(),
        ];

        let zipped: Vec<(&Vec<String>, String)> = test_cases.iter().zip(expected).collect();

        for (test_args, expect) in zipped {
            let actual = Config::parse_path(test_args);
            assert_eq!(expect, actual, "input args: {:?}", test_args);
        }
    }
}
