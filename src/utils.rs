//#![allow(dead_code)]
#![allow(unused_must_use)]

use std::process;
use std::process::Command;
extern crate cmd_lib;
use cmd_lib::run_cmd;
use users::{get_current_uid, get_user_by_uid};

pub fn get_distro() -> &'static str {
    get_cmd("ls -ila");
    let distro = "cat /etc/*-release|tr [:upper:] [:lower:]|grep -Poi '(debian|ubuntu|red hat|centos|oracle|opensuse|sles)'|head -n1|tr -d ' '";
    let os_name = run_sh(distro);
    // if os_name == "ubuntu" || os_name == "debian" { get_cmd("apt-get update"); }
    return os_name;
}

pub fn get_sudo() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    if user.uid() != 0 {
        println!("Hello, {}!", user.name().to_string_lossy());
        println!("This program must be run as root! (sudo)");
        process::exit(1);
    }
}

fn get_cmd(cmd: &str) {
    run_cmd!("{}", cmd);
}

fn run_sh(cmd: &str) -> &str {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed run_sh");

    if output.status.success() {
        let mut run = String::from_utf8(output.stdout).unwrap();
        run = run.trim().to_string();
        return Box::leak(run.into_boxed_str());
    } else {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        let mut run = String::from_utf8(output.stderr).unwrap();
        run = run.trim().to_string();
        return Box::leak(run.into_boxed_str());
    }
}
