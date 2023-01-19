use serde_json::json;
use std::error::Error;

use handlebars::{handlebars_helper, Handlebars};

/**
* Generate erc20 snippet too be added to custom contract file.
* Snipppet is a temlpate generated using handlebars
*
* @param openzeppelin: Bool representing if openzeppelin imports should be added
* @param contract_type: string representing the type of contract, eg. ERC20, ERC721, Custom, etc.
**/
pub fn generate_snippet(
    openzeppelin: bool,
    is_pauseable: bool,
    is_ownable: bool,
    is_reguarded: bool,
    contract_type: &str,
    contract_name: &str,
) -> String {
    let mut handlebars = Handlebars::new();

    handlebars_helper!(isDefined: |name: String| !name.is_empty());

    handlebars.register_helper("isDefined", Box::new(isDefined));

    let erc20_template = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract {{#if (isDefined contract_name)}}{{contract_name}}{{else}}MyContract{{/if}} is ERC20 {
    {{#if (isDefined contract_name)}}
    constructor(uint256 initialSupply) ERC20("{{contract_name}}", "{{contract_name}}") {
    {{else}}
    constructor(uint256 initialSupply) ERC20("GOLD", "GLD") {
        _mint(msg.sender, initialSupply);
    }
}
{{/if}} 
"#;

    let erc721_template = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract {{#if (isDefined contract_name)}}{{contract_name}}{{else}}GameItem{{/if}} is ERC721URIStorage {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    {{#if (isDefined contract_name)}}
    constructor() ERC721("{{contract_name}}", "{{contract_name}}") {}
    {{else}}
    constructor() ERC721("GameItem", "ITM") {}

    function awardItem(address player, string memory tokenURI)
        public
        returns (uint256)
    {
        uint256 newItemId = _tokenIds.current();
        _mint(player, newItemId);
        _setTokenURI(newItemId, tokenURI);

        _tokenIds.increment();
        return newItemId;
    }
}
"#;

    let erc1155_template = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";

contract {{#if (isDefined contract_name)}}{{contract_name}}{{else}}GameItems{{/if}} is ERC1155 {
    uint256 public constant GOLD = 0;
    uint256 public constant SILVER = 1;
    uint256 public constant THORS_HAMMER = 2;
    uint256 public constant SWORD = 3;
    uint256 public constant SHIELD = 4;

    constructor() ERC1155("https://game.example/api/item/{id}.json") {
        _mint(msg.sender, GOLD, 10**18, "");
        _mint(msg.sender, SILVER, 10**27, "");
        _mint(msg.sender, THORS_HAMMER, 1, "");
        _mint(msg.sender, SWORD, 10**9, "");
        _mint(msg.sender, SHIELD, 10**9, "");
    }
}
"#;

    let custom_template = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

{{#if isPauseable}}import "@openzeppelin/contracts/security/Pausable.sol";{{/if}}
{{#if isOwnable}}import "@openzeppelin/contracts/access/Ownable.sol";{{/if}}
{{#if isREGuarded}}import "@openzeppelin/contracts/security/ReentrancyGuard.sol";{{/if}}

contract {{#if (isDefined contract_name)}}{{contract_name}}{{else}}MyContract{{/if}}
{{#if initializable}}Initializable{{/if}},
{{#if pausable_upgradeable}}PausableUpgradeable{{/if}},
{{#if ownable_upgradeable}}OwnableUpgradeable{{/if}} {
    //do something
}
"#;

    let template = match contract_type {
        "erc20" => erc20_template,
        "erc721" => erc721_template,
        "erc1155" => erc1155_template,
        "custom" => custom_template,
        _ => return "Invalid contract type".to_string(),
    };

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
    handlebars
        .render(
            "erc20",
            &json!({ "openzeppelin": openzeppelin, "isPauseable":is_pauseable, "isOwnable":is_ownable, "isREGuarded":is_reguarded, "contract_name": contract_name }),
        )
        .unwrap()
}

/*  use handlebars::Handlebars;

let mut handlebars = Handlebars::new();

handlebars.register_template_string("my_template", "contract MyContract is
{{#if initializable}}Initializable{{/if}},
{{#if pausable_upgradeable}}PausableUpgradeable{{/if}},
{{#if ownable_upgradeable}}OwnableUpgradeable{{/if}} {
    do something
}").unwrap();

let data = json!({
    "initializable": true,
    "pausable_upgradeable": false,
    "ownable_upgradeable": true
});

let rendered = handlebars.render("my_template", &data).unwrap();

println!("{}", rendered);
*/
