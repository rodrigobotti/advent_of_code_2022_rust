use std::{env, path::PathBuf};

pub fn input_file_path(file_name: &str) -> PathBuf {
    env::current_dir().unwrap().join("input").join(file_name)
}

pub fn print_separator() {
    println!("{}", "-".repeat(50));
}
