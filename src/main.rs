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
