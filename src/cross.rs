use std::path::Path;
use pathbuf::pathbuf;
use const_format::formatcp;
use crate::{TEMP_PATH, utils::{download, extract_tar}};

const CROSS_FOLDER : &'static str =  formatcp!("{}/i486-linux-musl-cross", TEMP_PATH);
const CROSS_URL : &'static str = "https://musl.cc/i486-linux-musl-cross.tgz";

pub fn get_cross(){
    let cross_folder_path = Path::new(CROSS_FOLDER);
    if cross_folder_path.exists(){
        return;
    }
    let out_path = pathbuf![TEMP_PATH, "i486-linux-musl-cross.tgz"];
    download(CROSS_URL, &out_path);
    extract_tar(&out_path);
}