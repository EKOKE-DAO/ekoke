// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Ekoke is ERC20, Ownable {
    /// @notice The address of the reward pool contract.
    address public rewardPool;

    /// @notice The maximum supply of the token. (8 Millions and more)
    uint256 public constant MAX_SUPPLY = 888_010_101_000_000;
    /// @notice The maximum amount of tokens mintable by the reward pool. (66%)
    uint256 public constant MAX_REWARD_POOL_MINT = 592_006_734_000_000;
    /// @notice The maximum amount of tokens mintable by the owner. (33%)
    uint256 public constant MAX_OWNER_MINT = MAX_SUPPLY - MAX_REWARD_POOL_MINT;
    /// @notice contract decimals
    uint8 private DECIMALS = 8;

    /// @notice The amount of tokens minted by the reward pool.
    uint256 public rewardPoolMintedSupply = 0;
    /// @notice The amount of tokens minted by the owner.
    uint256 public ownerMintedSupply = 0;

    /// @notice Event emitted when tokens are minted by the reward pool.
    event RewardMinted(address indexed _to, uint256 _amount);
    /// @notice Event emitted when tokens are minted by the owner.
    event OwnerMinted(address indexed _to, uint256 _amount);

    constructor(
        address _initialOwner
    ) ERC20("Ekoke", "EKOKE") Ownable(_initialOwner) {
        rewardPool = address(0);
    }

    modifier onlyRewardPool() {
        require(
            msg.sender == rewardPool && rewardPool != address(0),
            "Ekoke: caller is not the reward pool"
        );
        _;
    }

    function decimals() public view virtual override returns (uint8) {
        return DECIMALS;
    }

    /// @notice Mint the provided amount of tokens to the recipient. Only the reward pool can call this function.
    /// @param _to the address that will receive the ETH Ekoke tokens.
    /// @param _amount the amount of tokens to mint.
    /// @dev This function can only be called by the reward pool.
    function mintRewardTokens(
        address _to,
        uint256 _amount
    ) public onlyRewardPool {
        require(
            rewardPoolMintedSupply + _amount <= MAX_REWARD_POOL_MINT,
            "Ekoke: reward pool minting limit reached"
        );
        require(
            totalSupply() + _amount <= MAX_SUPPLY,
            "Ekoke: total supply will exceed the maximum supply"
        );

        // mint the tokens to the recipients
        _mint(_to, _amount);

        // increment the amount of tokens minted by the reward pool
        rewardPoolMintedSupply += _amount;

        emit RewardMinted(_to, _amount);
    }

    /// @notice Mints the provided amount of tokens to the recipient.
    /// This function is only callable by the owner. The maximum total amount of tokens that can be minted is `MAX_OWNER_MINT`.
    /// @param _recipient the address that will receive the ETH Ekoke tokens.
    /// @param _amount the amount of tokens to mint.
    function adminMint(address _recipient, uint256 _amount) public onlyOwner {
        require(
            ownerMintedSupply + _amount <= MAX_OWNER_MINT,
            "Ekoke: owner minting limit reached"
        );
        require(
            totalSupply() + _amount <= MAX_SUPPLY,
            "Ekoke: total supply will exceed the maximum supply"
        );

        // mint the tokens to the recipient
        _mint(_recipient, _amount);

        // increment the amount of tokens minted by the owner
        ownerMintedSupply += _amount;

        // emit the OwnerMinted event
        emit OwnerMinted(_recipient, _amount);
    }

    /// @notice Set the address of the reward pool contract.
    /// @dev Sets the address of the ekoke canister. The address can only be set once.
    /// @param _reward_pool_address The new address of the reward pool.
    function adminSetRewardPoolAddress(
        address _reward_pool_address
    ) public onlyOwner {
        rewardPool = _reward_pool_address;
    }
}
