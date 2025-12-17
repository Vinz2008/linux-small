use std::{fs, path::Path, process::Command, thread};
use const_format::formatcp;
use pathbuf::pathbuf;
use crate::{TEMP_PATH, consts::CONFIGS_PATH, git::git_clone};

const LINUX_PATH : &'static str = formatcp!("{}/linux", TEMP_PATH);
const LINUX_URL : &'static str = "https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git";
const LINUX_VERSION : &'static str = "v6.14.11";
const LINUX_IMAGE_PATH : &'static str = formatcp!("{}/bzImage", TEMP_PATH);
const LINUX_CONFIG_PATH_START : &'static str = formatcp!("{}/.config", CONFIGS_PATH);

pub fn compile_linux(){
    git_clone(LINUX_URL, Path::new(LINUX_PATH), Some(LINUX_VERSION));
    let linux_exe_path = Path::new(LINUX_IMAGE_PATH);
    if linux_exe_path.exists(){
        return;
    }
    let linux_config_path_end = pathbuf![LINUX_PATH, ".config"];
    fs::copy(LINUX_CONFIG_PATH_START, &linux_config_path_end).unwrap();

    let core_nb = thread::available_parallelism().unwrap();
    let mut build_cmd = Command::new("make");
    build_cmd.current_dir(linux_config_path_end.parent().unwrap());
    build_cmd.arg("ARCH=x86").arg("bzImage").arg(&format!("-j{}", core_nb));
    build_cmd.spawn().unwrap().wait().unwrap();

    let linux_final_image = pathbuf![LINUX_PATH, "arch", "x86", "boot", "bzImage"];
    fs::copy(linux_final_image, linux_exe_path).unwrap();
}