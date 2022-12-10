// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "builder pattern" method of creating arguments
// which the most flexible, but also most verbose.
extern crate clap;
use clap::{App, Arg, SubCommand};
use std::process::Command;
use std::{env, fs};

#[cfg(windows)]
pub const NPM: &'static str = "npx.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

//TODO: Read/Write to files in contract folder
//TODO: Add custom code to contract files
//TODO: Figure out how to dynamically fetch the import routes to openzeppelin contracts eg. import "@openzeppelin/contracts/..."

fn check_for_node() {
    Command::new(NPM)
        .arg("hardhat")
        .status()
        .expect("node failed to fetch version");
}

fn mkdir_cd(project: &str) -> std::io::Result<()> {
    fs::create_dir_all(project)?;

    env::set_current_dir(&project).unwrap();

    Ok(())
}

// fn get_dependencies() -> String {
//     let dependencies = fs::read_to_string("../src/install.txt");
//     return dependencies.unwrap();
// }

fn erc20(contract: &str, project: &str) {
    // let path = env::current_dir();

    mkdir_cd(project).unwrap();

    Command::new(NPM)
        .arg("hardhat")
        .status()
        .expect("node failed to fetch version");

    Command::new("npm.cmd")
        .args([
            "install",
            "--save-dev",
            &fs::read_to_string("../src/install.txt").unwrap(),
        ])
        .status()
        .expect("An error occured while installing dependencies");

    // println!(
    //     "From the erc20 function: {:?}, {}, {:?}",
    //     path,
    //     contract,
    //     dependencies.as_ref()
    // );
}

fn erc721() {
    println!("From the erc721 function");
}

fn custom() {
    println!("From the custom domain...");
}

//? Try args

fn main() {
    let matches = App::new("Smart Contract Bootstrapper")
        .version("1.0")
        .author("Godswill E. <godswillezeoke@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("TYPE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PROJECT_NAME")
                .help("Set the project name")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .takes_value(true)
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short('d')
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    // Same as above examples...
    println!("{} :Hello there", matches.value_of("TYPE").unwrap());

    // match
    let name = matches.value_of("TYPE").unwrap();
    let project_name = matches.value_of("PROJECT_NAME").unwrap();

    match matches.value_of("TYPE").unwrap() {
        "node" => check_for_node(),
        "ERC20" => erc20(name, project_name),
        "ERC721" => erc721(),
        "Custom" => custom(),
        _ => println!("Don't be crazy!"),
    }
}
