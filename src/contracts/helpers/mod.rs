use indicatif::ProgressBar;
use std::fs::{copy, File};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use std::{env, fs};
mod template;

#[cfg(windows)]
pub const NPM: &'static str = "yarn.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

/**
* 1. Change current working directory to contracts/
* 2. Fetch contract snippet from generate_snippet function
* 3. Create file with provided filename and write the snippet from 2, into the file.
*
* @param filename: name of solidity file
* @param openzeppelin: if openzeppelin imports should be included
* @param contract_type: type of contract being created
**/
pub fn change_dir_and_make_file(
    filename: &str,
    openzeppelin: bool,
    contract_type: &str,
) -> std::io::Result<()> {
    env::set_current_dir("contracts").unwrap();

    let snippet = template::generate_snippet(openzeppelin, contract_type);

    let mut file = File::create(filename).unwrap();
    file.write_all(snippet.as_bytes()).unwrap();
    Ok(())
}

/**
* Creates a new folder for the hardhat project and switch to it as the cwd.
*
* @param project: refers to the project directory name
**/
pub fn mkdir_cd(project: &str) -> std::io::Result<()> {
    fs::create_dir_all(project)?;

    copy("./src/contracts/helpers/install.txt", "./Test/install.txt")?;
    env::set_current_dir(&project).unwrap();

    Ok(())
}

/**
* Installs all dependencies listed in "install.txt"
**/
pub fn install_dependencies() -> std::io::Result<()> {
    let dependencies = File::open("./install.txt").unwrap();
    let reader = BufReader::new(dependencies);

    // TODO: Dynamically get the number of packages in the install.txt file and pass it into ProgressBar::new()
    let bar = ProgressBar::new(5);

    // ? Try parallel iterators from rayon here
    for line in reader.lines() {
        let line = line.expect("Failed to read line.");

        Command::new(NPM)
            .arg("add")
            .arg("-D")
            .arg(line)
            .arg("--silent")
            .arg("--no-progress")
            .status()
            .expect("An error occured while installing dependency: {line}");

        bar.inc(1);
    }
    bar.finish();

    Ok(())
}
