use std::{env, fs, process::Command};

use pathbuf::pathbuf;

use crate::consts::FILESYSTEM_PATH;

// TODO : create a folder structure in a folder and just copy it ?

const CREATE_FOLDERS : &[&'static str] = &[
    "dev",
    "proc",
    "etc/init.d",
    "sys",
    "tmp",
    "home",
];

struct NodFile {
    path : &'static str,
    major : u8,
    minor : u8,
}

const CREATES_NOD : &[NodFile] = &[
    NodFile {
        path: "dev/console",
        major: 5,
        minor: 1,
    },
    NodFile {
        path: "dev/null",
        major: 1,
        minor: 3,
    }
];

struct File {
    path : &'static str,
    content : &'static str,
    is_exe : bool,
}

const CREATE_FILES : &[File] = &[
    File {
        path: "welcome",
        content: "Your welome message or ASCII art",
        is_exe: false,
    },
    File {
        path: "etc/inittab",
        content: "::sysinit:/etc/init.d/rc
::askfirst:/bin/sh
::restart:/sbin/init
::ctrlaltdel:/sbin/reboot
::shutdown:/bin/umount -a -r",
        is_exe: false,
    },
    File {
        path: "etc/init.d/rc",
        content: "#!/bin/sh
mount -t proc none /proc
mount -t sysfs none /sys
mdev -s
ln -s /proc/mounts /etc/mtab
mkdir -p /mnt /home
mount -t msdos -o rw /dev/fd0 /mnt
mkdir -p /mnt/data
mount --bind /mnt/data /home
clear
cat welcome
cd /home
/bin/sh",
        is_exe: false,
    }
];

fn is_running_graphically() -> bool {
    env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok() || matches!(env::var("XDG_SESSION_TYPE").as_deref(), Ok("x11") | Ok("wayland"))
}

pub fn init_filesystem(){
    for &folder in CREATE_FOLDERS {
        let folder_path = pathbuf![FILESYSTEM_PATH, folder];
        if !folder_path.exists(){
            fs::create_dir_all(folder_path).unwrap();
        }
    }

    let root_program = if is_running_graphically() {
        "pkexec"
    } else {
        "sudo"
    };

    for nod in CREATES_NOD {
        let nod_path = pathbuf![FILESYSTEM_PATH, nod.path];
        if !nod_path.exists() {
            let mut cmd = Command::new(root_program);
            cmd.arg("mknod").arg(nod_path).arg("c").arg(nod.major.to_string()).arg(nod.minor.to_string());
            cmd.spawn().unwrap().wait().unwrap();
        }
    }
}