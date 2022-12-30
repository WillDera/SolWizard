use std::process::Command;
mod helpers;

/**
 * Triggers an ERC20 contract creation
 * contract_type: contract type to be created i.e ERC20 (might be redundant)
 * project_name: the project name i.e. DeraDAO
 * filename: the contract file name i.e. DeraDAO.sol
 * */

pub fn erc20(contract_type: &str, project_name: &str, filename: &str) {
    helpers::mkdir_cd(project_name).unwrap();

    Command::new("npx.cmd")
        .arg("hardhat")
        .status()
        .expect("node failed to fetch version");

    // helpers::install_dependencies().unwrap();

    helpers::change_dir_and_make_file(filename).unwrap();
}

pub fn erc721() {
    println!("From the erc721 function");
}

pub fn custom() {
    println!("From the custom domain...");
}
