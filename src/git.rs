use std::{path::Path, process::{Command, Stdio}};

pub fn git_clone(url : &str, path : &Path, branch : Option<&str>){
    if path.exists(){
        return;
    }
    let mut cmd = Command::new("git");
    let path_str = path.to_str().unwrap();
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    cmd.arg("clone").arg("--depth=1");
    if let Some(branch) = branch {
        cmd.arg("--branch").arg(branch);
    }
    cmd.arg(url).arg(path_str);
    cmd.spawn().unwrap().wait().expect("failed to run git clone");
}