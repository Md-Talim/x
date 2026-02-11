mod languages;
mod runner;

use std::env;
use std::path::Path;
use std::process::{self};

use crate::runner::get_runner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: x <file.kt|file.java|file.cpp>");
        process::exit(1);
    }

    let file = &args[1];
    let ext = Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let Some(runner) = get_runner(ext) else {
        eprintln!("Unsupported file type: .{ext}");
        process::exit(1);
    };

    runner.run(&file);
}
