# Examples

> ## NB: All examples will have a project name of `Showcase`

1. Generate an ERC20 file named Test.sol

    `solwizard -t erc20 -f Test.sol -p Showcase`

2. Generate more than one ERC20 file

    `solwizard -t erc20 -f Test.sol Test1.sol -p Showcase`

3. Generate an ERC721 file named TestNFT.sol

    `solwizard -t erc721 -f TestNFT.sol -p Showcase`

    NB: The process of creating multiple ERC20 files in 2. above applies to 3.

4. Generate an ERC1155 file named TestNFT.sol

    `solwizard -t erc1155 -f TestNFT.sol -p Showcase`

    NB: The process of creating multiplem erc20 files in 2. above applies to 4.

5. Generate a Non-Reentrant ERC20 file named Test.sol

    `solwind -t erc20 -f Test.sol -p Showcase -isREGuarded`

    NB: Other available tags can be found in the help manual.
