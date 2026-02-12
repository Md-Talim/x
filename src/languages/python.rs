use crate::runner::{Runner, exec};

pub struct Python;

impl Runner for Python {
    fn run(&self, file: &str) {
        // Windows uses `python`, Unix uses `python3`
        if cfg!(windows) {
            exec("python", &[file]);
        } else {
            exec("python3", &[file]);
        }
    }
}
