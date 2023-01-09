use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::fs::{copy, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs};
mod template;

#[cfg(windows)]
pub const YARN: &'static str = "yarn.cmd";

#[cfg(not(windows))]
pub const YARN: &'static str = "yarn";

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
    filenames: Vec<&str>,
    openzeppelin: bool,
    contract_types: Vec<&str>,
) -> std::io::Result<()> {
    env::set_current_dir("contracts").unwrap();

    for x in 0..filenames.len() {
        let snippet = if contract_types.len() == 1 {
            template::generate_snippet(openzeppelin, contract_types[0])
        } else {
            template::generate_snippet(openzeppelin, contract_types[x])
        };

        let mut file = File::create(filenames[x]).unwrap();
        file.write_all(snippet.as_bytes()).unwrap();
    }

    Ok(())
}

/**
* Creates a new folder for the hardhat project and switch to it as the cwd.
*
* @param project: refers to the project directory name
**/
pub fn mkdir_cd(project: &str) -> std::io::Result<()> {
    let dest_path = Path::new(project).display().to_string();
    fs::create_dir_all(&dest_path)?;

    copy(
        "./src/create/helpers/install.txt",
        &format!("./{}/install.txt", &dest_path),
    )?;

    match env::set_current_dir(&dest_path) {
        Ok(()) => {
            Command::new(YARN)
                .arg("init")
                .arg("-y")
                .status()
                .expect("Couldnt initialize yarn project");

            Command::new(YARN)
                .arg("add")
                .arg("-D")
                .arg("hardhat")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()?;
        }
        Err(e) => {
            println!("Error changing current directory: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/**
* Installs all dependencies listed in "install.txt"
**/
pub fn install_dependencies() -> std::io::Result<()> {
    let dependencies = File::open("./install.txt").unwrap();
    let reader = BufReader::new(dependencies);

    let pb = ProgressBar::new(5);

    pb.set_style(
        ProgressStyle::default_bar()
            .progress_chars("##--")
            .template(
                "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .expect("Invalid progress bar template"),
    );

    let mut static_line = Box::new(String::new());

    for line in reader.lines() {
        pb.inc(1);
        let line = line.expect("Failed to read line.");
        *static_line = line.to_string();
        let message = format!("Installing {}", static_line);
        pb.set_message(message);

        Command::new(YARN)
            .arg("add")
            .arg("-D")
            .arg(line)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()?;
    }
    pb.finish_with_message("Dependencies installed");

    Ok(())
}
