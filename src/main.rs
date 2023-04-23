extern crate clap;
use clap::{Arg, ArgAction, Command};
use fancy::printcoln;
use std::process::Command as cmd;
mod create;

#[cfg(windows)]
pub const NPX: &str = "npx.cmd";

#[cfg(not(windows))]
pub const NPX: &str = "npx";

fn main() {
    let matches = Command::new("Solidity Wizard")
        .version("1.0.0")
        .author("Godswill E. <godswillezeoke@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("contract_category")
                .short('c')
                .long("contract category")
                .help("The category of a contract. eg. Normal or Custom")
                .default_value("normal")
                .num_args(1)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("contract_type")
                .short('t')
                .long("contract type")
                .help("Sets the contract type(s) to create")
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .help("Sets the contract filename(s) i.e. <filename>.sol")
                .num_args(1..)
                .action(ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("project_name")
                .short('p')
                .long("project name")
                .action(ArgAction::Set)
                .help("Sets the project name")
                .required(true),
        )
        .arg(
            Arg::new("openzeppelin")
                .short('z')
                .long("openzeppelin")
                .help("Use openzeppelin standard and imports")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("isPauseable")
                .long("Pauseable")
                .help("Make contract Pauseable")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("isOwnable")
                .long("Ownable")
                .help("Make contract Ownable")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("isREGuarded")
                .long("ReEGuard")
                .help("Make contract NonReEntrant")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // match
    let contract_type: Vec<&str> = matches
        .get_many::<String>("contract_type")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let filenames: Vec<&str> = matches
        .get_many::<String>("filename")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    // * You can replace contains_id with get_flag -> examples/tutorial_builder/03_01_flag_bool.rs
    let project_name = matches.get_one::<String>("project_name").unwrap();
    let openzeppelin = matches.get_flag("openzeppelin");
    let is_pauseable = matches.get_flag("isPauseable");
    let is_ownable = matches.get_flag("isOwnable");
    let is_reguarded = matches.get_flag("isREGuarded");

    let npx = cmd::new(crate::NPX).arg("--version").output();

    match npx {
        Ok(_) => {
            printcoln!("[bold|green]==> Npx Found...");
        }
        Err(_) => {
            printcoln!("[bold|red]==> Npx required for Hardhat!");
            std::process::exit(1);
        }
    }

    assert!(
        contract_type.len() == 1 || contract_type.len() == filenames.len(),
        "🚫 Must have atleast 1 contract type OR same number of contract types as filenames provided!"
    );

    match matches
        .get_one::<String>("contract_category")
        .expect("Contract category required")
        .as_ref()
    {
        "normal" => create::normal(
            contract_type,
            project_name,
            filenames,
            openzeppelin,
            is_pauseable,
            is_ownable,
            is_reguarded,
        ),
        "custom" => create::custom(
            contract_type,
            project_name,
            filenames,
            openzeppelin,
            is_pauseable,
            is_ownable,
            is_reguarded,
        ),
        _ => println!("Don't be crazy!"),
    }
}
