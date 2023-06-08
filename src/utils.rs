use std::{env, path::PathBuf};

pub fn input_file_path(file_name: &str) -> PathBuf {
    env::current_dir().unwrap().join("input").join(file_name)
}

pub fn print_separator() {
    println!("{}", "-".repeat(50));
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
