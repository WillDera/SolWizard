// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "builder pattern" method of creating arguments
// which the most flexible, but also most verbose.
extern crate clap;
use clap::{App, Arg};
use std::process::Command;
mod helpers;

//TODO: Read/Write to files in contract folder
//TODO: Add custom code to contract files
//TODO: Figure out how to dynamically fetch the import routes to openzeppelin contracts eg. import "@openzeppelin/contracts/..."

fn check_for_node() {
    println!("Hello from node");
}

// fn install_dependencies() -> std::io::Result<()> {
//     let dependencies = File::open("../src/install.txt").unwrap();
//     let reader = BufReader::new(dependencies);

//     for line in reader.lines() {
//         let line = line.expect("Failed to read line.");

//         Command::new(NPM)
//             .args(["install", "--save-dev"])
//             .arg(line)
//             .status()
//             .expect("An error occured while installing dependency: {line}");
//     }

//     Ok(())
// }

fn erc20(contract: &str, project: &str, filename: &str) {
    // let path = env::current_dir();

    helpers::mkdir_cd(project).unwrap();

    Command::new("npx.cmd")
        .arg("hardhat")
        .status()
        .expect("node failed to fetch version");

    helpers::install_dependencies().unwrap();

    // change_dir_and_make_file()

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
            Arg::with_name("type")
                .short('t')
                .help("Sets the contract type to create")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filename")
                .short('f')
                .help("Sets the contract filename i.e. <filename>.sol")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project_name")
                .short('p')
                .takes_value(true)
                .help("Sets the project name"),
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
        .get_matches();

    // Same as above examples...
    println!("{} :Hello there", matches.value_of("type").unwrap());

    // match
    let name = matches.value_of("type").unwrap().to_lowercase();
    let project_name = matches.value_of("project_name").unwrap();
    let filename = matches.value_of("filename").unwrap();

    match matches.value_of("type").unwrap() {
        "node" => check_for_node(),
        "erc20" => erc20(&name, project_name, filename),
        "erc721" => erc721(),
        "Custom" => custom(),
        _ => println!("Don't be crazy!"),
    }
}
