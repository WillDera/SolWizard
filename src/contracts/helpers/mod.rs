use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use std::{env, fs};
mod template;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

pub fn change_dir_and_make_file(filename: &str) -> std::io::Result<()> {
    env::set_current_dir("contracts").unwrap();

    let snippet = template::generate_erc20_snippet();

    let mut file = File::create(filename).unwrap();
    file.write_all(snippet.as_bytes()).unwrap();
    Ok(())
}

pub fn mkdir_cd(project: &str) -> std::io::Result<()> {
    fs::create_dir_all(project)?;

    env::set_current_dir(&project).unwrap();

    Ok(())
}

pub fn install_dependencies() -> std::io::Result<()> {
    let dependencies = File::open("./install.txt").unwrap();
    let reader = BufReader::new(dependencies);

    // ? Try parallel iterators from rayon here
    for line in reader.lines() {
        let line = line.expect("Failed to read line.");

        Command::new(NPM)
            .args(["install", "--quiet", "--no-progress", "--save-dev"])
            .arg(line)
            .status()
            .expect("An error occured while installing dependency: {line}");
    }

    Ok(())
}
