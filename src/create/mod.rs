use fancy::printcoln;
use std::path::Path;
use std::process::Command;
mod helpers;
pub mod util;
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
    is_pauseable: bool,
    is_ownable: bool,
    is_reguarded: bool,
) {
    // if project_name is an empty string or hardhat.config.json is abscent, then execute the following step.
    if !Path::new("hardhat.config.json").exists() || !project_name.is_empty() {
        helpers::mkdir_cd(project_name).unwrap();

        printcoln!("[bold|green]==> Starting Hardhat...");
        Command::new(util::NPX)
            .arg("hardhat")
            .status()
            .expect("Filed to start hardhat!");
    }

    // If OS is windows, go ahead and install dependencies from the install.txt file
    // If OS is not windows, delete the install.txt file which was copied in the previous stage above
    #[cfg(windows)]
    if Path::new("install.txt").exists() {
        helpers::install_dependencies().unwrap();
    }

    helpers::change_dir_and_make_file(
        filenames,
        openzeppelin,
        is_pauseable,
        is_ownable,
        is_reguarded,
        contract_types,
    )
    .unwrap();

    printcoln!("[bold|green]==> Execution Complete!");
}

/**
 * Triggers Custom contract creation
 * @param contract_type: contract type to be created i.e ERC20 (might be redundant)
 * @param project_name: the project name i.e. DeraDAO
 * @param filename: the contract file name i.e. DeraDAO.sol
 * @param openzeppelin: if openzeppelin imports should be included
 * */
pub fn custom(
    _contract_types: Vec<&str>,
    _project_name: &str,
    _filenames: Vec<&str>,
    _openzeppelin: bool,
    _is_pauseable: bool,
    _is_ownable: bool,
    _is_reguarded: bool,
) {
    // if !Path::new("hardhat.config.json").exists() {
    //     helpers::mkdir_cd(project_name).unwrap();

    //     Command::new(util::NPX)
    //         .arg("hardhat")
    //         .status()
    //         .expect("Failed to start hardhat!");
    // }

    // helpers::install_dependencies().unwrap();

    // helpers::change_dir_and_make_file(
    //     filenames,
    //     openzeppelin,
    //     is_pauseable,
    //     is_ownable,
    //     is_reguarded,
    //     contract_types,
    // )
    // .unwrap();
    printcoln!("[bold|yellow]==> Custom contracts in development");
}
