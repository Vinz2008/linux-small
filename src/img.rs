use std::fs;
use crate::{consts::{CONFIGS_PATH, TEMP_PATH}, utils::chmod_x};

use const_format::formatcp;

const SYSLINUX_CONFIG_START : &'static str = formatcp!("{}/syslinux.cfg", CONFIGS_PATH);
const SYSLINUX_CONFIG_END : &'static str = formatcp!("{}/syslinux.cfg", TEMP_PATH);


// TODO : copy the filesystem to a temp_filesystem folder and then chown to root the content and then run cpio
// TODO : copy the syslinux.cfg
pub fn generate_img(){
    fs::copy(SYSLINUX_CONFIG_START, SYSLINUX_CONFIG_END).unwrap();
    chmod_x(SYSLINUX_CONFIG_END).unwrap();
}