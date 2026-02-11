use crate::runner::{Runner, exec};

pub struct Python;

impl Runner for Python {
    fn run(&self, file: &str) {
        exec("python3", &[file]);
    }
}
