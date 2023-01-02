extern crate clap;
use clap::{App, Arg};
mod contracts;

fn check_for_node() {
    println!("Hello from node");
}

fn main() {
    let matches = App::new("Smart Contract Bootstrapper")
        .version("1.0")
        .author("Godswill E. <godswillezeoke@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("file_count")
                .short('n')
                .help("Number of files to be generated")
                .takes_value(true)
                .default_value("single"),
        )
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
        .arg(Arg::with_name("openzeppelin").action(clap::ArgAction::SetTrue))
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
    let contract_type = matches.value_of("type").unwrap().to_lowercase();
    let project_name = matches.value_of("project_name").unwrap();
    let filename = matches.value_of("filename").unwrap();
    let openzeppelin = matches.contains_id("openzeppelin");

    match matches.value_of("file_count").unwrap() {
        "node" => check_for_node(), // just for testing, remove later
        "single" => contracts::contract(&contract_type, project_name, filename, openzeppelin),

        // TODO: multiple should take number of files, filenames and filetypes (contract type) to be generated
        // "multiple" => contracts::contracts(number_of_files, project_name, filenames, openzeppelin),
        // "Custom" => contracts::custom(),
        _ => println!("Don't be crazy!"),
    }
}
