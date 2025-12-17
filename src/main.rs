use std::{fs, path::Path, time::Instant};

use crate::{busybox::compile_busybox, consts::TEMP_PATH, cross::get_cross, linux::compile_linux};

mod consts;
mod git;
mod utils;
mod cross;
mod linux;
mod busybox;

fn main() {
    let start = Instant::now();
    let temp_path =  Path::new(TEMP_PATH);
    if !temp_path.exists() {
        fs::create_dir(temp_path).expect("Couldn't create the temp folder");
    }
    get_cross();
    compile_linux();
    compile_busybox();
    println!("Took {}s", start.elapsed().as_secs());
}
