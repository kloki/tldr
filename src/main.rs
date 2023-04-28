use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Verify the files in a path don't exceed a max amount of lines
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// Path to files to verify.
    #[arg(env = "TLDR_PATH")]
    paths: Vec<PathBuf>,
    /// Max amount of lines per file.
    #[arg(short, long, default_value_t = 1000, env = "TLDR_MAX_LINES")]
    max_lines: usize,
    /// If set, only files matching the pattern will be checked
    #[arg(short, long, default_value_t = String::from(""), env = "TLDR_INCLUDE_PATTERN")]
    include_pattern: String,
    /// If set, all files matching the pattern will be igored
    #[arg(short, long, default_value_t = String::from(""), env = "TLDR_EXCLUDE_PATTERN")]
    exclude_pattern: String,
}

fn get_size(path: &PathBuf) -> usize {
    match fs::read_to_string(path) {
        Ok(s) => s.lines().collect::<Vec<_>>().len(),
        Err(_) => 0,
    }
}

fn red(message: &str) -> String {
    format!("\x1b[0;31m{}\x1b[0m", message)
}

fn exit(message: String) {
    println!("{}", red(&message));
    std::process::exit(1)
}

fn check_files(args: Args) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
    let mut files: Vec<PathBuf> = args
        .paths
        .iter()
        .map(|p| {
            WalkDir::new(p)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.into_path())
                .collect::<Vec<PathBuf>>()
        })
        .collect::<Vec<_>>()
        .concat();
    if args.include_pattern != "" {
        let re = Regex::new(&args.include_pattern)?;
        files.retain(|e| re.is_match(e.to_str().unwrap_or("")));
    }
    if args.exclude_pattern != "" {
        let re = Regex::new(&args.exclude_pattern)?;
        files.retain(|e| !re.is_match(e.to_str().unwrap_or("")));
    }
    let failing_files = files
        .iter()
        .map(|p| (p.display().to_string(), get_size(&p)))
        .filter(|p| p.1 > args.max_lines)
        .collect::<Vec<_>>();
    Ok(failing_files)
}

fn main() {
    let args = Args::parse();
    match check_files(args) {
        Err(e) => exit(e.to_string()),
        Ok(failing_files) => {
            if failing_files.len() == 0 {
                println!("All fine! 🎉");
                return;
            }
            println!(
                "{}{}",
                red("No good! 👮 🚨 Some files are too long."),
                failing_files
                    .iter()
                    .map(|p| format!("\n {}:{}", p.0, p.1))
                    .collect::<String>()
            );
            std::process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_success() {
        let tmp_dir = TempDir::new("/tmp/testing").unwrap();
        let file_path_1 = tmp_dir.path().join("python.py");
        let mut tmp_file_1 = File::create(file_path_1).unwrap();
        writeln!(tmp_file_1, "1\n2\n3\n").unwrap();
        let failing_files = check_files(Args {
            paths: vec![PathBuf::from(tmp_dir.path())],
            include_pattern: "".to_string(),
            exclude_pattern: "".to_string(),
            max_lines: 4,
        })
        .unwrap();
        assert!(failing_files.len() == 0)
    }

    #[test]
    fn test_to_long() {
        let tmp_dir = TempDir::new("/tmp/testing").unwrap();
        let file_path_1 = tmp_dir.path().join("python.py");
        let mut tmp_file_1 = File::create(file_path_1).unwrap();
        writeln!(tmp_file_1, "1\n2\n3\n").unwrap();
        let failing_files = check_files(Args {
            paths: vec![PathBuf::from(tmp_dir.path())],
            include_pattern: "".to_string(),
            exclude_pattern: "".to_string(),
            max_lines: 1,
        })
        .unwrap();
        assert!(failing_files.len() == 1)
    }
    #[test]
    fn test_include_pattern() {
        let tmp_dir = TempDir::new("/tmp/testing").unwrap();
        let file_path_1 = tmp_dir.path().join("python.py");
        let mut tmp_file_1 = File::create(file_path_1).unwrap();
        writeln!(tmp_file_1, "1\n2\n3\n").unwrap();
        let file_path_2 = tmp_dir.path().join("test.txt");
        let mut tmp_file_2 = File::create(file_path_2).unwrap();
        writeln!(tmp_file_2, "1\n2\n3\n").unwrap();
        let failing_files = check_files(Args {
            paths: vec![PathBuf::from(tmp_dir.path())],
            include_pattern: "py$".to_string(),
            exclude_pattern: "".to_string(),
            max_lines: 1,
        })
        .unwrap();
        assert!(failing_files.len() == 1)
    }
    #[test]
    fn test_exclude_pattern() {
        let tmp_dir = TempDir::new("/tmp/testing").unwrap();
        let file_path_1 = tmp_dir.path().join("python.py");
        let mut tmp_file_1 = File::create(file_path_1).unwrap();
        writeln!(tmp_file_1, "1\n2\n3\n").unwrap();
        let file_path_2 = tmp_dir.path().join("test.txt");
        let mut tmp_file_2 = File::create(file_path_2).unwrap();
        writeln!(tmp_file_2, "1\n2\n3\n").unwrap();
        let failing_files = check_files(Args {
            paths: vec![PathBuf::from(tmp_dir.path())],
            include_pattern: "".to_string(),
            exclude_pattern: "(py$)|(txt$)".to_string(),
            max_lines: 1,
        })
        .unwrap();
        assert!(failing_files.len() == 0)
    }
    #[test]
    fn test_bad_regex() {
        let result = check_files(Args {
            paths: vec![PathBuf::new()],
            include_pattern: "\\".to_string(),
            exclude_pattern: "".to_string(),
            max_lines: 1,
        });
        assert!(result.is_err());
        let result = check_files(Args {
            paths: vec![PathBuf::new()],
            include_pattern: "".to_string(),
            exclude_pattern: "\\".to_string(),
            max_lines: 1,
        });
        assert!(result.is_err());
    }
}
