// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Fly is ERC20, Ownable {
    address private fly_canister_address;
    uint8 private _decimals;
    uint256 public swapFee;

    uint256 private constant GOERLI_CHAIN_ID = 5;
    uint256 private constant HARDHAT_CHAIN_ID = 31337;

    event FlySwapped(
        address indexed _from,
        bytes32 indexed _principal,
        uint256 _amount
    );

    constructor(
        address _initialOwner,
        uint256 _swapFee
    ) ERC20("Fly", "FLY") Ownable(_initialOwner) {
        _decimals = 12;
        swapFee = _swapFee;
        fly_canister_address = address(0);
    }

    modifier onlyFlyCanister() {
        require(
            msg.sender == fly_canister_address &&
                fly_canister_address != address(0),
            "Fly: caller is not the fly canister"
        );
        _;
    }

    modifier onlyTestnet() {
        require(
            block.chainid == GOERLI_CHAIN_ID ||
                block.chainid == HARDHAT_CHAIN_ID,
            "Fly: this function can only be called on testnets"
        );
        _;
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    function totalSupply() public view virtual override returns (uint256) {
        return 8_880_101_010_000_000_000;
    }

    /**
     * @dev Returns the total supply of tokens swapped from Ethereum blockchain to IC. Basically it's the circulating supply of the token on Ethereum.
     * @return The total supply of tokens swapped from Ethereum blockchain to IC.
     */
    function swappedSupply() public view returns (uint256) {
        return ERC20.totalSupply();
    }

    /**
     * @dev Returns the address of the fly canister.
     * @return The address of the fly canister.
     */
    function getFlyCanisterAddress() public view returns (address) {
        require(
            fly_canister_address != address(0),
            "Fly: fly canister address not set"
        );
        return fly_canister_address;
    }

    /**
     * @dev Sets the address of the fly canister. The address can only be set once.
     * @param _fly_canister_address The new address of the fly canister.
     */
    function setFlyCanisterAddress(
        address _fly_canister_address
    ) public onlyOwner {
        require(
            fly_canister_address == address(0),
            "Fly: fly canister address already set"
        );
        fly_canister_address = _fly_canister_address;
    }

    /**
     * @dev Sets the swap fee.
     * @param _swapFee The new swap fee.
     */
    function setSwapFee(uint256 _swapFee) public onlyOwner {
        swapFee = _swapFee;
    }

    /**
     * @dev Swaps the Fly tokens from Ethereum blockchain to IC from the caller to the recipient principal for the provided amount.
     * @param _recipient The principal to receive the tokens.
     * @param _amount The amount of tokens to swap.
     */
    function swap(bytes32 _recipient, uint256 _amount) public payable {
        // check if the fly canister address is set
        require(
            fly_canister_address != address(0),
            "Fly: fly canister address not set"
        );
        // check if the caller has enough tokens to swap
        require(
            balanceOf(msg.sender) >= _amount,
            "Fly: caller does not have enough tokens to swap"
        );
        // check if the caller has enough ether to pay the fee
        require(
            msg.value >= swapFee,
            "Fly: caller does not have enough ether to pay the fee"
        );
        // burn the tokens from the caller
        _burn(msg.sender, _amount);
        // emit swap event
        emit FlySwapped(msg.sender, _recipient, _amount);
        // pay fee
        payable(fly_canister_address).transfer(msg.value);
    }

    /**
     * @dev Mints the provided amount of tokens to the recipient (after a swap on the fly canister).
     * @param _recipient the address that will receive the ETH Fly tokens.
     * @param _amount the amount of tokens to mint.
     */
    function transcribeSwap(
        address _recipient,
        uint256 _amount
    ) public onlyFlyCanister {
        // mint the tokens to the recipient
        _mint(_recipient, _amount);
    }

    /**
     * @dev Mints the provided amount of tokens to the recipient. This function is only callable on testnets.
     * @param _recipient the address that will receive the ETH Fly tokens.
     * @param _amount the amount of tokens to mint.
     */
    function mintTestnetTokens(
        address _recipient,
        uint256 _amount
    ) public onlyTestnet onlyOwner {
        _mint(_recipient, _amount);
    }
}
