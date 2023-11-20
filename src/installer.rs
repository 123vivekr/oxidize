use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn cargo_install(package_name: &str) {
    let mut cmd = Command::new("cargo");
    cmd.arg("install").arg(package_name);

    let output = cmd.output().expect("Failed to execute command");
    cmd.stderr(Stdio::inherit()).stdin(Stdio::inherit());

    if output.status.success() {
        println!("Package '{}' successfully installed.", package_name);
    } else {
        println!("Error installing package '{}': {:?}", package_name, output);
    }
}

pub fn update_shell_config(alias_str: Vec<String>) {
    let path_str = env::var("PATH").unwrap();
    let home_path = env::var("HOME").unwrap();
    let cargo_path = format!("{}/.cargo/bin", home_path);

    let mut paths_vec: Vec<&str> = path_str.split(':').collect();
    if let Some(index) = paths_vec.iter().position(|&path| path == cargo_path) {
        paths_vec.remove(index);
    }

    let shell_rc = match env::var("SHELL").as_deref() {
        Ok("/bin/zsh") => "zshrc",
        Ok("/bin/bash") => "bashrc",
        _ => panic!("Unsupported shell"),
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/.{}", home_path, shell_rc))
        .unwrap();

    for alias in &alias_str {
        if let Err(e) = writeln!(file, "{}", alias) {
            eprintln!("Faile writing to file: {}", e);
        }
    }

    if let Err(e) = writeln!(file, "export PATH=$HOME/.cargo/bin:$PATH") {
        eprintln!("Faile writing to file: {}", e);
    };
}
