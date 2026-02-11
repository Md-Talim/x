use crate::runner::{Runner, cleanup, exec};

pub struct Cpp;

impl Runner for Cpp {
    fn run(&self, file: &str) {
        let out = "main";

        exec("g++", &[file, "-o", out]);
        exec(&format!("./{}", out), &[]);

        cleanup(out);
    }
}
