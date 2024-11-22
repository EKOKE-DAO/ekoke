// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Ekoke} from "./Ekoke.sol";

contract RewardPool is Ownable {
    /// @notice The address of the EKOKE token
    address private ekoke;

    /// @notice The address of the Deferred ERC721 token
    address private deferred;

    /// @notice The address of marketplace
    address private marketplace;

    /// @notice The amount of EKOKE tokens reserved for the reward pool
    uint256 public reservedAmount;

    /// @notice Event emitted when the reward pool is reserved
    event PoolReserved(uint256 reward, uint256 tokens);

    /// @notice Event emitted when the reward is sent
    event RewardSent(address to, uint256 amount);

    modifier onlyMarketplace() {
        require(
            msg.sender == marketplace && marketplace != address(0),
            "RewardPool: caller is not the marketplace"
        );
        _;
    }

    modifier onlyDeferred() {
        require(
            msg.sender == deferred && deferred != address(0),
            "RewardPool: caller is not deferred"
        );
        _;
    }

    constructor(
        address _owner,
        address _ekoke,
        address _deferred
    ) Ownable(_owner) {
        require(_ekoke != address(0), "RewardPool: EKOKE address is zero");
        require(
            _deferred != address(0),
            "RewardPool: Deferred address is zero"
        );
        ekoke = _ekoke;
        deferred = _deferred;
        reservedAmount = 0;
    }

    /// @notice Reserve the reward pool
    /// @param _reward The amount of EKOKE as a reward for a token
    /// @param _tokens The amount of deferred tokens to reserve
    function reservePool(
        uint256 _reward,
        uint256 _tokens
    ) external onlyDeferred {
        require(_reward > 0, "RewardPool: reward is zero");
        uint256 totalAmount = _reward * _tokens;

        Ekoke ekokeToken = Ekoke(ekoke);

        // check if we have enough EKOKE tokens to reserve
        uint256 rewardAlreadyMinted = ekokeToken.rewardPoolMintedSupply();
        uint256 maximumRewardSupply = ekokeToken.MAX_REWARD_POOL_MINT();

        require(
            rewardAlreadyMinted + totalAmount <= maximumRewardSupply,
            "RewardPool: reward pool has not enough liquidity"
        );

        reservedAmount += totalAmount;

        emit PoolReserved(_reward, _tokens);
    }

    /// @notice send reward to the provided address
    /// @dev only the marketplace can call this function
    /// @param _to The address to send the reward
    /// @param _amount The amount of EKOKE tokens to send
    function sendReward(address _to, uint256 _amount) external onlyMarketplace {
        require(_amount > 0, "RewardPool: amount is zero");
        require(
            reservedAmount >= _amount,
            "RewardPool: not enough reserved amount"
        );

        // mint reward tokens
        Ekoke(ekoke).mintRewardTokens(_to, _amount);

        // decrease the reserved amount
        reservedAmount -= _amount;

        // emit event
        emit RewardSent(_to, _amount);
    }

    /// @notice Get the available reward in the reward pool
    /// @return available the amount of EKOKE tokens available for the reward
    function availableReward() external view returns (uint256 available) {
        uint256 mintedAmount = Ekoke(ekoke).rewardPoolMintedSupply();
        uint256 maximumRewardSupply = Ekoke(ekoke).MAX_REWARD_POOL_MINT();

        return maximumRewardSupply - (mintedAmount + reservedAmount);
    }

    /// @notice Set the address of the marketplace
    /// @param _marketplace The address of the marketplace
    function adminSetMarketplace(address _marketplace) external onlyOwner {
        marketplace = _marketplace;
    }
}
