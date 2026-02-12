use crate::runner::{Runner, cleanup, exec};

pub struct C;
pub struct Cpp;

fn compile_and_run(compiler: &str, file: &str) {
    let (out, run) = if cfg!(windows) {
        ("main.exe", "main.exe")
    } else {
        ("main", "./main")
    };

    exec(compiler, &[file, "-o", out]);
    exec(run, &[]);

    cleanup(out);
}

impl Runner for C {
    fn run(&self, file: &str) {
        compile_and_run("gcc", file);
    }
}

impl Runner for Cpp {
    fn run(&self, file: &str) {
        compile_and_run("g++", file);
    }
}
