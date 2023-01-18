use std::path::Path;
use std::process::Command;
mod helpers;

#[cfg(windows)]
pub const HARDHAT: &str = "hardhat.cmd";

#[cfg(not(windows))]
pub const HARDHAT: &str = "hardhat";

/**
 * Triggers Normal contract creation
 * @param contract_type: contract type to be created i.e ERC20 (might be redundant)
 * @param project_name: the project name i.e. DeraDAO
 * @param filename: the contract file name i.e. DeraDAO.sol
 * @param openzeppelin: if openzeppelin imports should be included
 * */

pub fn normal(
    contract_types: Vec<&str>,
    project_name: &str,
    filenames: Vec<&str>,
    openzeppelin: bool,
    isPauseable: bool,
    isOwnable: bool,
    isREGuarded: bool,
) {
    // if project_name is an empty string or hardhat.config.json is abscent, then execute the following step.
    if !Path::new("hardhat.config.json").exists() || !project_name.is_empty() {
        helpers::mkdir_cd(project_name).unwrap();

        Command::new(HARDHAT)
            .status()
            .expect("Filed to start hardhat!");
    }

    if Path::new("install.txt").exists() && !Path::new("package.json").exists() {
        helpers::install_dependencies().unwrap();
    }

    // If OS is windows, go ahead and install dependencies from the install.txt file
    // If OS is not windows, delete the install.txt file which was copied in the previous stage above
    match std::env::consts::OS {
        "windows" => helpers::install_dependencies().unwrap(),
        _ => std::fs::remove_file("install.txt").unwrap(),
    }

    helpers::change_dir_and_make_file(
        filenames,
        openzeppelin,
        isPauseable,
        isOwnable,
        isREGuarded,
        contract_types,
    )
    .unwrap();
}

/**
 * Triggers Custom contract creation
 * @param contract_type: contract type to be created i.e ERC20 (might be redundant)
 * @param project_name: the project name i.e. DeraDAO
 * @param filename: the contract file name i.e. DeraDAO.sol
 * @param openzeppelin: if openzeppelin imports should be included
 * */
pub fn contracts(
    contract_types: Vec<&str>,
    project_name: &str,
    filenames: Vec<&str>,
    openzeppelin: bool,
    isPauseable: bool,
    isOwnable: bool,
    isREGuarded: bool,
) {
    if !Path::new("hardhat.config.json").exists() {
        helpers::mkdir_cd(project_name).unwrap();

        Command::new(HARDHAT)
            .status()
            .expect("Failed to start hardhat!");
    }

    let foo = String::new();
    helpers::install_dependencies().unwrap();

    helpers::change_dir_and_make_file(
        filenames,
        openzeppelin,
        isPauseable,
        isOwnable,
        isREGuarded,
        contract_types,
    )
    .unwrap();
}
