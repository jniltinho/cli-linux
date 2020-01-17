use std::process::Command;

pub fn get_distro() -> String {
    let output = Command::new("sh")
                         .arg("-c")
                         .arg("cat /etc/*-release|tr [:upper:] [:lower:]|grep -Poi '(debian|ubuntu|red hat|centos|oracle|opensuse|sles)'|head -n1|tr -d ' '")
                         .output()
                         .expect("Failed to get Distro info");

    let distro = String::from_utf8(output.stdout).unwrap();
    return distro.trim().to_string();
    //println!("{}", distro);
}
