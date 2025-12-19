use const_format::formatcp;

pub const TEMP_PATH : &'static str = "./temp";
pub const CONFIGS_PATH : &'static str = "./configs";
pub const FILESYSTEM_PATH : &'static str = formatcp!("{}/filesystem", TEMP_PATH);