extern crate clap;
use clap::{App, Arg, ArgAction};
mod create;

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
                .long("file count")
                .help("Number of files to be generated")
                .takes_value(true)
                .default_value("single"),
        )
        .arg(
            Arg::with_name("type")
                .short('t')
                .long("contract type")
                .help("Sets the contract type(s) to create")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::with_name("filename")
                .short('f')
                .long("filename")
                .help("Sets the contract filename(s) i.e. <filename>.sol")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::with_name("project_name")
                .short('p')
                .long("project name")
                .takes_value(true)
                .help("Sets the project name"),
        )
        .arg(
            Arg::with_name("openzeppelin")
                .short('o')
                .long("openzeppelin")
                .help("Use openzeppelin standard and imports")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    // Same as above examples...
    println!("{} :Hello there", matches.value_of("type").unwrap());

    // match
    let contract_type: Vec<&str> = matches
        .get_many::<String>("type")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    let project_name = matches.value_of("project_name").unwrap();
    let filename: Vec<&str> = matches
        .get_many::<String>("filename")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    let openzeppelin = matches.contains_id("openzeppelin");

    match matches.value_of("file_count").unwrap() {
        "node" => check_for_node(), // just for testing, remove later
        "single" => create::contract(contract_type[0], project_name, filename[0], openzeppelin),

        // TODO: multiple should take number of files, filenames and filetypes (contract type) to be generated
        "multiple" => create::contracts(contract_type, project_name, filename, openzeppelin),
        // "Custom" => contracts::custom(),
        _ => println!("Don't be crazy!"),
    }
}
