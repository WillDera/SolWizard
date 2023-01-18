extern crate clap;
use clap::{App, Arg, ArgAction};
mod create;

fn main() {
    let matches = App::new("Smart Contract Bootstrapper")
        .version("1.0.0")
        .author("Godswill E. <godswillezeoke@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("contract_category")
                .short('c')
                .long("contract category")
                .help("The category of a contract. eg. Normal or Custom")
                .takes_value(true)
                .default_value("normal"),
        )
        .arg(
            Arg::with_name("contract_type")
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
                .short('z')
                .long("openzeppelin")
                .help("Use openzeppelin standard and imports")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("isPauseable")
                .short('p')
                .long("Pauseable")
                .help("Make contract Pauseable")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("isOwnable")
                .short('o')
                .long("Pauseable")
                .help("Make contract Ownable")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("isREGuarded")
                .short('r')
                .long("ReEntrancy Guard")
                .help("Make contract NonReEntrant")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    // match
    let contract_type: Vec<&str> = matches
        .get_many::<String>("contract_type")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let filename: Vec<&str> = matches
        .get_many::<String>("filename")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let project_name = matches.value_of("project_name").unwrap_or("");
    let openzeppelin = matches.contains_id("openzeppelin");
    let isPauseable = matches.contains_id("isPauseable");
    let isOwnable = matches.contains_id("isOwnable");
    let isREGuarded = matches.contains_id("isREGuarded");

    match matches.value_of("contract_category").unwrap() {
        "normal" => create::normal(
            contract_type,
            project_name,
            filename,
            openzeppelin,
            isPauseable,
            isOwnable,
            isREGuarded,
        ),
        "custom" => create::contracts(
            contract_type,
            project_name,
            filename,
            openzeppelin,
            isPauseable,
            isOwnable,
            isREGuarded,
        ),
        _ => println!("Don't be crazy!"),
    }
}
