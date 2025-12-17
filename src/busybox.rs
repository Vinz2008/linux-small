use const_format::formatcp;

const BUSYBOX_VERSION : &'static str = "1_36_1";
const BUSYBOX_URL : &'static str = formatcp!("https://github.com/mirror/busybox/archive/refs/tags/{}.tar.gz", BUSYBOX_VERSION);

pub fn compile_busybox(){
    //download(BUSYBOX_URL, &out_path);
    todo!()
}