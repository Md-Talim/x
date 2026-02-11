use std::process::{self, Command};

pub trait Runner {
    fn run(&self, file: &str);
}

/// Execute a command, exit on failure.
pub fn exec(program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to execute: `{}`: {}", program, e);
            process::exit(1);
        });

    if !status.success() {
        eprintln!("`{} {}` failed", program, args.join(" "));
        process::exit(1);
    }
}

/// Clean up a file, ignoring errors.
pub fn cleanup(path: &str) {
    let _ = std::fs::remove_file(path);
}

use crate::languages::*;

pub fn get_runner(ext: &str) -> Option<Box<dyn Runner>> {
    match ext {
        "c" => Some(Box::new(cpp::C)),
        "cpp" | "cc" => Some(Box::new(cpp::Cpp)),
        "java" => Some(Box::new(java::Java)),
        "kt" => Some(Box::new(kotlin::Kotlin)),
        "py" => Some(Box::new(python::Python)),
        _ => None,
    }
}
