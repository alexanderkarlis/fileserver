mod utils;

use std::io;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let greeting = format!("Hello, {}!", name);
    alert(&greeting);
}

pub fn visit_dirs(dir: &Path, hash_map: <HashMap>, counter: usize) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let counter_new = counter + 1;
            if path.is_dir() {
                visit_dirs(&path, counter_new)?;
            } else {
                let file = format!("{}{:#?}", "\t".repeat(counter_new), &entry.path());
                println!("{}", file);
            }
        }
    }
    Ok(())
}

use std::io;
use std::path::Path;
use std::fs;
// use std::collections::HashMap;
// use walkdir::{DirEntry}
// DirEntry.
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn visit_dirs<'a>(dir: &Path, path_list: &'a mut Vec<String>, counter: usize) -> io::Result<&'a mut Vec<String>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let counter_new = counter + 1;
            let entry = entry?;
            let path = entry.path();
            println!("{:#?}", path.as_path());
            if path.is_dir() {
                visit_dirs(&path, path_list, counter_new)?;
            } else {
                let path_str = format!("{}", path.display());
                path_list.push(path_str);
                let file = format!("{}{:#?}", "\t".repeat(counter_new), path);
                println!("{}", file);
            }
        }
    }
    Ok(path_list)
}

fn main() {
    let dir = Path::new("/home/alexanderkarlis/test");
    // let mut path_list: Vec<String> = Vec::new();
    let path_list = &mut Vec::new();
    let spacing: usize = 0;
    let a = visit_dirs(&dir, path_list, spacing);
    println!("{:#?}", a)
}
