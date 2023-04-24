use clap::Parser;
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
}

fn get_size(path: &PathBuf) -> usize {
    match fs::read_to_string(path) {
        Ok(s) => s.lines().collect::<Vec<_>>().len(),
        Err(_) => 0,
    }
}

fn main() {
    let args = Args::parse();

    let recursive_files: Vec<PathBuf> = args
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
    let failing_files = recursive_files
        .iter()
        .map(|p| (p, get_size(&p)))
        .filter(|p| p.1 > args.max_lines)
        .collect::<Vec<_>>();
    if failing_files.len() == 0 {
        println!("All fine! 🎉")
    } else {
        let output = failing_files
            .iter()
            .map(|p| format!("\n• {}:{}", p.0.display(), p.1))
            .collect::<String>();
        println!("\x1b[0;31mNo good!\x1b[0m 👮 🚨 {}", output);
        std::process::exit(1);
    }
}
