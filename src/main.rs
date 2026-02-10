use std::path::Path;
use std::process::{self, Command};
use std::{env, fs};

fn run_cmd(program: &str, args: &[&str]) {
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

fn run_cpp(file: &str) {
    let out = "main";

    run_cmd("g++", &[file, "-o", out]);
    run_cmd(&format!("./{}", out), &[]);

    let _ = fs::remove_file(out);
}

fn run_java(file: &str) {
    let path = Path::new(file);
    let class_name = path
        .file_stem()
        .expect("invalid file name")
        .to_str()
        .expect("non UTF-8 file name");

    run_cmd("javac", &[file]);
    run_cmd("java", &[class_name]);

    let class_file = format!("{}.class", class_name);
    let _ = fs::remove_file(&class_file);
}

fn run_kotlin(file: &str) {
    let out = "out.jar";

    run_cmd("kotlinc", &[file, "-include-runtime", "-d", out]);
    run_cmd("java", &["-jar", out]);

    let _ = fs::remove_file(&out);
}

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

    match ext {
        "cpp" | "cc" => run_cpp(file),
        "java" => run_java(file),
        "kt" => run_kotlin(file),
        _ => {
            eprintln!("File not supported");
            process::exit(1);
        }
    }
}
