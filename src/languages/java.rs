use crate::runner::{Runner, cleanup, exec};
use std::path::Path;

pub struct Java;

impl Runner for Java {
    fn run(&self, file: &str) {
        let class_name = Path::new(file)
            .file_stem()
            .expect("invalid file name")
            .to_str()
            .expect("non UTF-8 file name");

        exec("javac", &[file]);
        exec("java", &[class_name]);

        cleanup(&format!("{class_name}.class"));
    }
}
