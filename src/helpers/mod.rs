use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::{env, fs};

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

pub fn change_dir_and_make_file(name: &str, folder: &str) -> std::io::Result<()> {
    env::set_current_dir(&folder).unwrap();
    let mut file = File::create(name)?;
    Ok(())
}

pub fn mkdir_cd(project: &str) -> std::io::Result<()> {
    fs::create_dir_all(project)?;

    env::set_current_dir(&project).unwrap();

    Ok(())
}

pub fn install_dependencies() -> std::io::Result<()> {
    let dependencies = File::open("../src/install.txt").unwrap();
    let reader = BufReader::new(dependencies);

    for line in reader.lines() {
        let line = line.expect("Failed to read line.");

        Command::new(NPM)
            .args(["install", "--save-dev"])
            .arg(line)
            .status()
            .expect("An error occured while installing dependency: {line}");
    }

    Ok(())
}
