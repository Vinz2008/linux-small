use std::{fs, path::Path, process::Command, thread};
use const_format::{concatcp, formatcp};
use pathbuf::pathbuf;
use crate::{utils::{download, extract_tar}, consts::{CONFIGS_PATH, TEMP_PATH, FILESYSTEM_PATH}};

const BUSYBOX_VERSION : &'static str = "1_36_1";
const BUSYBOX_TAR_NAME : &'static str = formatcp!("{}.tar.gz", BUSYBOX_VERSION);
const BUSYBOX_URL : &'static str = formatcp!("https://github.com/mirror/busybox/archive/refs/tags/{}", BUSYBOX_TAR_NAME);
const BUSYBOX_FOLDER_NAME : &'static str = formatcp!("busybox-{}", BUSYBOX_VERSION);
const BUSYBOX_PATH : &'static str = concatcp!(TEMP_PATH, "/", BUSYBOX_FOLDER_NAME);
const BUSYBOX_CONFIG_PATH_START : &'static str = formatcp!("{}/busybox/.config", CONFIGS_PATH);


pub fn compile_busybox(){
    let filesystem_path = Path::new(FILESYSTEM_PATH);
    if filesystem_path.exists(){
        return;
    }
    let out_path = pathbuf![TEMP_PATH, BUSYBOX_TAR_NAME];
    download(BUSYBOX_URL, &out_path);
    extract_tar(&out_path);

    let busybox_config_path_end = pathbuf![BUSYBOX_PATH, ".config"];
    fs::copy(BUSYBOX_CONFIG_PATH_START, busybox_config_path_end).unwrap();

    // TODO : put the core nb in a context struct passed everywhere
    let core_nb = thread::available_parallelism().unwrap();
    let mut build_cmd = Command::new("make");
    build_cmd.current_dir(BUSYBOX_PATH);
    build_cmd.arg("ARCH=x86").arg(&format!("-j{}", core_nb));

    build_cmd.spawn().unwrap().wait().unwrap();

    let mut install_cmd = Command::new("make");
    install_cmd.current_dir(BUSYBOX_PATH);
    install_cmd.arg("ARCH=x86").arg("install");
    install_cmd.spawn().unwrap().wait().unwrap();

    let install_busybox_path = pathbuf![BUSYBOX_PATH, "_install"];
    fs::rename(install_busybox_path, filesystem_path).unwrap();
}