use std::{path::Path, process::{Command, Stdio}};


// TODO : use request and indicatif ?
pub fn download(url : &str, out_path : &Path){
    let mut cmd = Command::new("wget");

    cmd.arg("-O").arg(out_path).arg(url);

    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    cmd.spawn().unwrap().wait().expect("failed to download");
}

pub fn extract_tar(path : &Path){
    // TODO : other way ?
    let filename = path.file_name().unwrap();
    let mut cmd = Command::new("tar");
    cmd.arg("xvf").arg(filename);

    cmd.current_dir(path.parent().unwrap());
    
    cmd.spawn().unwrap().wait().expect("failed to extract tar");
}