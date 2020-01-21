//#![allow(dead_code)]
#![allow(unused_must_use)]

use std::process;
use std::process::Command;
extern crate cmd_lib;
use cmd_lib::{run_cmd, run_fun, FunResult};
use users::{get_current_uid, get_user_by_uid};

pub fn get_distro() -> String {
    get_cmd("ls -ila");
    let distro = "cat /etc/*-release|tr [:upper:] [:lower:]|grep -Poi '(debian|ubuntu|red hat|centos|oracle|opensuse|sles)'|head -n1|tr -d ' '";
    //println!("{}", _run_sh(distro).unwrap());
    let os_name = run_sh2(distro);

    //if os_name == "ubuntu" || os_name == "debian" { get_cmd("apt-get update"); }
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

pub fn get_date() -> String {
    extern crate chrono;
    use chrono::{DateTime, Local};
    let now: DateTime<Local> = Local::now();
    //println!("Local now is: {}", now);
    //println!("Local now in RFC 2822 is: {}", now.to_rfc2822());
    //println!("Local now in RFC 3339 is: {}", now.to_rfc3339());
    now.format("%A, %e de %B de %Y as %H:%M").to_string()
}

fn _run_sh(cmd: &str) -> FunResult {
    let cmd_run = format!("/bin/sh -c \"{}\"", cmd);
    return run_fun!("{}", cmd_run);
}

fn get_cmd(cmd: &str) {
    run_cmd!("{}", cmd);
}

fn run_sh2(cmd: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed run_sh");

    if output.status.success() {
        let run = String::from_utf8(output.stdout).unwrap();
        return run.trim().to_string();
    } else {
        let run = String::from_utf8(output.stderr).unwrap();
        return run.trim().to_string();
    }
}
