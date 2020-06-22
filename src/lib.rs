mod utils;

use std::io;
use std::path::Path;
use std::fs;
use wasm_bindgen::prelude::*;
use web_sys::console;

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

// fn wrapper() -> String {

//     console::log(&list_dirs());
// }

#[wasm_bindgen]
pub fn list_dirs() -> String {
    let dir_path = "../";
    let dir = Path::new(dir_path);
    println!("checking path {:#?}", dir_path);
    let path_list = &mut Vec::new();
    let spacing: usize = 0;
    let dirs_result = check_dirs(&dir, path_list, spacing);
    println!("{:#?}", dirs_result);
    
    let all_dirs = match dirs_result {
        Ok(dirs) => serde_json::to_string(&dirs).unwrap(),
        _ => serde_json::to_string(&path_list).unwrap()
    };
    let logged = format!("Checking path, {:#?}!", dir_path);
    console::log_1(&logged.into());
    all_dirs
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn check_dirs<'a>(dir: &Path, path_list: &'a mut Vec<String>, counter: usize) -> io::Result<&'a mut Vec<String>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let counter_new = counter + 1;
            let entry = entry?;
            let path = entry.path();
            println!("{:#?}", path.as_path());
            if path.is_dir() {
                check_dirs(&path, path_list, counter_new)?;
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
