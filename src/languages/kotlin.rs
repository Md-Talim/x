use crate::runner::{Runner, cleanup, exec};

pub struct Kotlin;

impl Runner for Kotlin {
    fn run(&self, file: &str) {
        let out = "out.jar";

        exec("kotlinc", &[file, "-include-runtime", "-d", out]);
        exec("java", &["-jar", out]);

        cleanup(out);
    }
}
