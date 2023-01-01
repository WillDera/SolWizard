use serde_json::json;
use std::error::Error;

use handlebars::Handlebars;

/**
* Generate erc20 snippet too be added to custom contract file.
* Snipppet is a temlpate generated using handlebars
**/
pub fn generate_erc20_snippet() -> String {
    let mut handlebars = Handlebars::new();

    let template = r#"
    pragma solidity ^0.8.0;

    import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

    contract TokenName is ERC20 {
      /* 
      TokenName: Eg., The DAO Token
      TokenTicker: Eg., TDT
      */ 
      constructor(uint256 initialSupply) ERC20("TokenName", "TokenTicker") {
          _mint(msg.sender, initialSupply);
      }
    }
  "#;

    //  register template string and catch errors if any
    handlebars
        .register_template_string("erc20", template)
        .unwrap_or_else(|e| {
            println!("Error compiling template: {}", e);
            if let Some(source) = e.source() {
                println!("Source: {}", source);
            }
        });
    //  render the template without a closing semicolon -> this would return a string to match the function's return type.
    handlebars.render("erc20", &()).unwrap()
}
