// nft.sol


pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract CustomNFT is ERC721 {
    constructor() ERC721("CustomNFT", "CNFT") {
        _mint(msg.sender, 1);
    }
}
