use std::process::Command;
mod helpers;

/**
 * Triggers contract creation
 * @param contract_type: contract type to be created i.e ERC20 (might be redundant)
 * @param project_name: the project name i.e. DeraDAO
 * @param filename: the contract file name i.e. DeraDAO.sol
 * @param openzeppelin: if openzeppelin imports should be included
 * */

pub fn contract(contract_type: &str, project_name: &str, filename: &str, openzeppelin: bool) {
    helpers::mkdir_cd(project_name).unwrap();

    Command::new("hardhat.cmd")
        .status()
        .expect("node failed to fetch version");

    // helpers::install_dependencies().unwrap();

    // helpers::change_dir_and_make_file(filename, openzeppelin, contract_type).unwrap();
}

pub fn contracts(
    contract_types: Vec<&str>,
    project_name: &str,
    filenames: Vec<&str>,
    openzeppelin: bool,
) {
    println!("{:?}", filenames);
}
