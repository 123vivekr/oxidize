use std::env;
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

pub fn set_path() {
    let path_str = env::var("PATH").unwrap();
    let home_path = env::var("HOME").unwrap();

    let predicate = |path: &&str| path == &format!("{}/.cargo/bin", home_path);

    let shell_rc = match env::var("SHELL").as_deref() {
        Ok("/bin/zsh") => "zshrc",
        Ok("/bin/bash") => "bashrc",
        _ => panic!("Unsupported shell"),
    };

    match path_str.split(':').find(predicate) {
        Some(_) => (),
        None => println!(
            "Run: echo \"export PATH=$HOME/.cargo/bin:$PATH\" > $HOME/.{} if using ",
            shell_rc
        ),
    }
}
