use indicatif::ProgressBar;
use std::fs::{copy, read_dir, File};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use std::{env, fs};
mod template;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

/*
* 1. Change current working directory to contracts/
* 2. Fetch contract snippet from generate_erc20_snippet function
* 3. Create file with provided filename and write the snippet from 2, into the file.
*/
pub fn change_dir_and_make_file(filename: &str) -> std::io::Result<()> {
    env::set_current_dir("contracts").unwrap();

    let snippet = template::generate_erc20_snippet();

    let mut file = File::create(filename).unwrap();
    file.write_all(snippet.as_bytes()).unwrap();
    Ok(())
}

/*
* Creates a new folder for the hardhat project and switch to it as the cwd.
*/
pub fn mkdir_cd(project: &str) -> std::io::Result<()> {
    fs::create_dir_all(project)?;

    copy("./src/contracts/helpers/install.txt", "./Test/install.txt")?;
    env::set_current_dir(&project).unwrap();

    Ok(())
}

/*
* Installs all dependencies listed in "install.txt"
*/
pub fn install_dependencies() -> std::io::Result<()> {
    let dependencies = File::open("./install.txt").unwrap();
    let reader = BufReader::new(dependencies);

    // TODO: Dynamically get the number of packages in the install.txt file and pass it into ProgressBar::new()
    let bar = ProgressBar::new(5);

    // ? Try parallel iterators from rayon here
    for line in reader.lines() {
        let line = line.expect("Failed to read line.");

        Command::new(NPM)
            .args(["install", "--quiet", "--no-progress", "--save-dev"])
            .arg(line)
            .status()
            .expect("An error occured while installing dependency: {line}");

        bar.inc(1);
    }
    bar.finish();

    Ok(())
}
