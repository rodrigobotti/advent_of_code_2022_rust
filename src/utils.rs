use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

pub fn print_separator() {
    println!("{}", "-".repeat(50));
}

fn input_file_path(file_name: &str) -> PathBuf {
    env::current_dir()
        .expect("failed to get current directory")
        .join("input")
        .join(file_name)
}

fn read_file_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_input_lines(file_name: &str) -> Lines<BufReader<File>> {
    let file_path = input_file_path(file_name);
    read_file_lines(file_path).expect("Failed to open file {file_path}")
}

pub fn read_line(line: io::Result<String>) -> String {
    line.expect("Failed to read line from buffer")
}

#[macro_export]
macro_rules! hashmap {
    // counting expressions: https://danielkeep.github.io/tlborm/book/blk-counting.html
    // counting using the same macro: https://docs.rs/maplit/1.0.2/src/maplit/lib.rs.html#1-324
    (@single $_t:tt $sub:expr) => { $sub };
    (@count $($tts:tt)*) => {<[()]>::len(&[$(super::hashmap!(@single $tts ())),*])};

    ( $( $key:expr  => $val:expr ),* ) => {
        {
            let cap = super::hashmap!(@count $($key)*);
            let mut hash_map = std::collections::HashMap::with_capacity(cap);
            $(
                hash_map.insert($key, $val);
            )*
            hash_map
        }
    };
}

pub(crate) use hashmap;
