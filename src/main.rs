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

fn main() {
    let args = Args::parse();

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
        match Regex::new(&args.include_pattern) {
            Ok(re) => files.retain(|e| re.is_match(e.to_str().unwrap())),
            Err(e) => {
                println!("\x1b[0;31m{}\x1b[0m", e);
                std::process::exit(1)
            }
        };
    }
    if args.exclude_pattern != "" {
        match Regex::new(&args.exclude_pattern) {
            Ok(re) => files.retain(|e| !re.is_match(e.to_str().unwrap())),
            Err(e) => {
                println!("\x1b[0;31m{}\x1b[0m", e);
                std::process::exit(1)
            }
        };
    }
    let failing_files = files
        .iter()
        .map(|p| (p, get_size(&p)))
        .filter(|p| p.1 > args.max_lines)
        .collect::<Vec<_>>();

    if failing_files.len() != 0 {
        let output = failing_files
            .iter()
            .map(|p| format!("\n {}:{}", p.0.display(), p.1))
            .collect::<String>();
        println!("\x1b[0;31mNo good!\x1b[0m 👮 🚨 {}", output);
        std::process::exit(1);
    }
    println!("All fine! 🎉")
}
