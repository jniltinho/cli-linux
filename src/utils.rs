//#![allow(dead_code)]
#![allow(unused_must_use)]

use std::process;
use std::process::Command;
extern crate cmd_lib;
use cmd_lib::run_cmd;
use users::{get_current_uid, get_user_by_uid};

pub fn get_distro() -> String {
    get_cmd(String::from("ls -ila"));
    let distro = String::from("cat /etc/*-release|tr [:upper:] [:lower:]|grep -Poi '(debian|ubuntu|red hat|centos|oracle|opensuse|sles)'|head -n1|tr -d ' '");
    let os_name = run_sh(distro);

    if os_name == "ubuntu" {
        get_cmd(String::from("apt-get update"));
    }
    return os_name;
}

fn get_cmd(cmd: String) {
    run_cmd!("{}", cmd);
}

fn run_sh(cmd: String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("Failed run_sh");

    let run = String::from_utf8(output.stdout).unwrap();
    return run.trim().to_string();
}

pub fn get_sudo() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    if user.uid() != 0 {
        println!("Hello, {}!", user.name().to_string_lossy());
        println!("This program must be run as root! (sudo)");
        process::exit(1);
    }
}
