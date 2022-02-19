use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("default_word_list.rs");

    let word_list: Vec<_> = include_str!("./dictionary.txt")
        .lines()
        // .map(|s| s.as_bytes().iter().copied().collect::<Vec<_>>()).flatten()
        .collect();

    let word_list_len = word_list.len();
    let word_list_slice = word_list.as_slice();

    fs::write(
        &dest_path,
        format!(
            r"
pub const DEFAULT_WORD_LIST: [&'static str; {word_list_len}] = {word_list_slice:?};"),
    )
        .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
}
