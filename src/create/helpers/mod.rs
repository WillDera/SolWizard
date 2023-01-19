use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{copy, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, fs};
mod template;

#[cfg(windows)]
pub const YARN: &str = "yarn.cmd";

#[cfg(not(windows))]
pub const YARN: &str = "yarn";

/**
* Creates a new folder for the hardhat project and switch to it as the cwd.
*
* @param project: refers to the project directory name
**/
pub fn mkdir_cd(project: &str) -> std::io::Result<()> {
    let dest_path = Path::new(project).display().to_string();
    fs::create_dir_all(&dest_path)?;

    #[cfg(windows)]
    copy(
        "./src/create/helpers/install.txt",
        format!("./{}/install.txt", &dest_path),
    )?;

    match env::set_current_dir(&dest_path) {
        Ok(()) => {
            Command::new(YARN)
                .arg("init")
                .arg("-y")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .expect("Failed to initialize yarn project");

            Command::new(YARN)
                .arg("add")
                .arg("-D")
                .arg("hardhat")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .expect("Failed to start hardhat");
        }
        Err(e) => {
            println!("Error changing current directory: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

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
    is_pauseable: bool,
    is_ownable: bool,
    is_reguarded: bool,
    contract_types: Vec<&str>,
) -> std::io::Result<()> {
    env::set_current_dir("contracts").unwrap();

    let contract_names: Vec<&str> = filenames
        .clone()
        .into_iter()
        .map(|f| f.split('.').next().unwrap())
        .collect();

    for x in 0..filenames.len() {
        let snippet = if contract_types.len() == 1 {
            template::generate_snippet(
                openzeppelin,
                is_pauseable,
                is_ownable,
                is_reguarded,
                contract_types[0],
                contract_names[x],
            )
        } else {
            template::generate_snippet(
                openzeppelin,
                is_pauseable,
                is_ownable,
                is_reguarded,
                contract_types[x],
                contract_names[x],
            )
        };

        let mut file = File::create(filenames[x]).unwrap();
        file.write_all(snippet.as_bytes()).unwrap();
    }

    Ok(())
}

/**
* Installs all dependencies listed in "install.txt"
**/
pub fn install_dependencies() -> std::io::Result<()> {
    let dependencies = File::open("./install.txt").unwrap();
    let reader = BufReader::new(&dependencies);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let pb = ProgressBar::new(lines.len() as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .progress_chars("##>-")
            .template(
                "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .expect("Invalid progress bar template"),
    );
    pb.enable_steady_tick(Duration::from_millis(5));

    let mut static_line = Box::<std::string::String>::default();

    for line in lines {
        pb.inc(1);

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

    // std::fs::remove_file("./install.txt").unwrap();

    Ok(())
}
