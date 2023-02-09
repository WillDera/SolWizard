# SolWizard

> A Solidity file generator which is built ontop of [Hardhat SDK](https://github.com/NomicFoundation/hardhat) to generate `.sol` files based on a user's requests right after Hardhat creates the project.

## Installation

To install SolWizard (not deployed yet), run the following command

    cargo install xxx

## Guide

You can find examples here:

-   [examples](examples/examples.md)

## How to run

1.  Create a project named "Test" with an "ERC20" solidity file named "name.sol"

        solwizard -t erc20 -f name.sol -p Test

    -   Supported contract types: [here](https://github.com/WillDera/SolWizard#supported-contract-types)

2.  Help

        solwizard --help

## Coming Soon

-   Creation of custom contracts.

## Supported contract types

| Contract type | Supported |
| :-----------: | :-------: |
|     ERC20     |    ✅     |
|    ERC721     |    ✅     |
|    ERC1155    |    ✅     |
