# Smart Contract Bootsrapper

This is a boostrapper built ontop of Hardhat SDK to cutdown setup time with regards to processes like creating a solidity file and writing basic snippets in those files. This is a process that is repeated continuously over time and I figured, having a tool to automate the process would be much better.

## Current Iteration

1. The tool currently only automates the creation of basic ERC20 solidity file as shown in the template.rs file.
2. The current implementation would require repetition to implement the ERC721, ERC1155 and so on contracts. Therefore, the use of handlebar's conditionals would be best depending on the arguements passed to the program.
3. The current implementation would always create a new hardhat project, therefore, a rule to check if hardhat.config.js exists would be needed. This would be used to decide if a new project should be created or not.

## How to run

1. Hreate a project named "Test" with an "ERC20" solidity file named "name.sol"

   `cargo run -- -t erc20 -f name.sol -p Test`

2. Help

   `cargo run -- --help`

## Improvements

- [x] Have one function to conditionally render a template depending on what type of contract is required.
- [ ] Use ArgAction::Append from clap to accept multiple filetypes and filenames.
- [ ] Check for `hardhat.config.js` before running other processes.
- [ ] If config file exists, carry out solidity file creation with expected templates.
- [ ] Implement a progress bar.

## Future Improvements

- [ ] Allow creation of more than one solidity file.
- [ ] Conditional renderer should render templates based on number of files and kinds of contract (including custom contracts)
