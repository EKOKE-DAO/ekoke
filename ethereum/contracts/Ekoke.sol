// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Ekoke is ERC20, Ownable {
    address private ekoke_ledger_canister_address;
    uint8 private _decimals;

    uint256 private constant GOERLI_CHAIN_ID = 5;
    uint256 private constant SEPOLIA_CHAIN_ID = 11155111;
    uint256 private constant HARDHAT_CHAIN_ID = 31337;

    event EkokeSwapped(
        address indexed _from,
        bytes32 indexed _principal,
        uint256 _amount
    );

    constructor(
        address _initialOwner
    ) ERC20("Ekoke", "EKOKE") Ownable(_initialOwner) {
        _decimals = 8;
        ekoke_ledger_canister_address = address(0);
    }

    modifier onlyEkokeLedgerCanister() {
        require(
            msg.sender == ekoke_ledger_canister_address &&
                ekoke_ledger_canister_address != address(0),
            "Ekoke: caller is not the ekoke canister"
        );
        _;
    }

    modifier onlyTestnet() {
        require(
            block.chainid == GOERLI_CHAIN_ID ||
                block.chainid == SEPOLIA_CHAIN_ID ||
                block.chainid == HARDHAT_CHAIN_ID,
            "Ekoke: this function can only be called on testnets"
        );
        _;
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    function totalSupply() public view virtual override returns (uint256) {
        return 888_010_101_000_000;
    }

    /**
     * @dev Returns the total supply of tokens swapped from Ethereum blockchain to IC. Basically it's the circulating supply of the token on Ethereum.
     * @return The total supply of tokens swapped from Ethereum blockchain to IC.
     */
    function swappedSupply() public view returns (uint256) {
        return ERC20.totalSupply();
    }

    /**
     * @dev Returns the address of the ekoke canister.
     * @return The address of the ekoke canister.
     */
    function getEkokeLedgerCanisterAddress() public view returns (address) {
        require(
            ekoke_ledger_canister_address != address(0),
            "Ekoke: ekoke canister address not set"
        );
        return ekoke_ledger_canister_address;
    }

    /**
     * @dev Sets the address of the ekoke canister. The address can only be set once.
     * @param _ekoke_canister_address The new address of the ekoke canister.
     */
    function setEkokeLedgerCanisterAddress(
        address _ekoke_canister_address
    ) public onlyOwner {
        require(
            ekoke_ledger_canister_address == address(0),
            "Ekoke: ekoke canister address already set"
        );
        ekoke_ledger_canister_address = _ekoke_canister_address;
    }

    /**
     * @dev Swaps the Ekoke tokens from Ethereum blockchain to IC from the caller to the recipient principal for the provided amount.
     * @param _recipient The principal to receive the tokens.
     * @param _amount The amount of tokens to swap.
     */
    function swap(bytes32 _recipient, uint256 _amount) public {
        // check if the ekoke canister address is set
        require(
            ekoke_ledger_canister_address != address(0),
            "Ekoke: ekoke canister address not set"
        );
        // check if the caller has enough tokens to swap
        require(
            balanceOf(msg.sender) >= _amount,
            "Ekoke: caller does not have enough tokens to swap"
        );
        // burn the tokens from the caller
        _burn(msg.sender, _amount);
        // emit swap event
        emit EkokeSwapped(msg.sender, _recipient, _amount);
    }

    /**
     * @dev Mints the provided amount of tokens to the recipient (after a swap on the ekoke canister).
     * @param _recipient the address that will receive the ETH Ekoke tokens.
     * @param _amount the amount of tokens to mint.
     */
    function transcribeSwap(
        address _recipient,
        uint256 _amount
    ) public onlyEkokeLedgerCanister {
        // mint the tokens to the recipient
        _mint(_recipient, _amount);
    }

    /**
     * @dev Mints the provided amount of tokens to the recipient. This function is only callable on testnets.
     * @param _recipient the address that will receive the ETH Ekoke tokens.
     * @param _amount the amount of tokens to mint.
     */
    function mintTestnetTokens(
        address _recipient,
        uint256 _amount
    ) public onlyTestnet onlyOwner {
        _mint(_recipient, _amount);
    }
}
