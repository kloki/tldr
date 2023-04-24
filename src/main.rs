use clap::Parser;

/// Verify the files in a path don't exceed a max amount of lines
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// Path to files to verify.
    #[arg(env = "TLDR_PATH")]
    path: String,

    /// Max amount of lines per file.
    #[arg(short, long, default_value_t = 10000, env = "TLDR_MAX_LINES")]
    max_lines: usize,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world! {} {}", args.path, args.max_lines);
}
