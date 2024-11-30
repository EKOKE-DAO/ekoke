// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Deferred} from "./Deferred.sol";
import {RewardPool} from "./RewardPool.sol";

contract Marketplace is Ownable {
    /// @notice The address of the USD ERC20 token
    address public usdErc20;

    /// @notice The address of the EKOKE token
    address private ekoke;

    /// @notice The address of the Deferred ERC721 token
    address private deferred;

    /// @notice Reward pool address
    address private rewardPool;

    /// @notice The interest rate for the contract buyers. So buyers pay 10% more than the token price
    /// the extra 10% will be locked in the marketplace contract
    uint8 public interestRate = 10;

    /// @notice tokens that has been sold
    mapping(uint256 => bool) private soldTokens;

    /// @notice Event emitted when a token is bought
    event TokenBought(
        address indexed buyer,
        address indexed seller,
        uint256 tokenId,
        uint256 price,
        uint256 paidAmount
    );

    constructor(
        address _owner,
        address _usdErc20,
        address _ekoke,
        address _deferred
    ) Ownable(_owner) {
        require(
            _usdErc20 != address(0),
            "Marketplace: USD ERC20 address is zero"
        );
        require(
            _deferred != address(0),
            "Marketplace: Deferred address is zero"
        );
        usdErc20 = _usdErc20;
        ekoke = _ekoke;
        deferred = _deferred;
    }

    /// @notice Set the address of the reward pool
    /// @param _rewardPool The address of the reward pool
    function adminSetRewardPool(address _rewardPool) external onlyOwner {
        rewardPool = _rewardPool;
    }

    /// @notice Set the interest rate for the contract buyers
    /// @param _interestRate The interest rate
    function adminSetInterestRate(uint8 _interestRate) external onlyOwner {
        require(
            _interestRate > 0,
            "Marketplace: Interest rate must be greater than 0"
        );
        require(
            _interestRate <= 100,
            "Marketplace: Interest rate must be less than 100"
        );
        interestRate = _interestRate;
    }

    /// @notice Buy a deferred NFT with the configured USD ERC20 token
    /// @param _tokenId The ID of the deferred NFT\
    function buyToken(uint256 _tokenId) external {
        require(rewardPool != address(0), "Marketplace: Reward pool not set");

        // get the contract from deferred
        Deferred deferredContract = Deferred(deferred);
        // get the contract for token
        Deferred.SellContract memory sellContract = deferredContract
            .tokenContract(_tokenId);

        // get token buyer
        address tokenBuyer = msg.sender;
        // get whether the buyer is a contract buyer
        bool _isContractBuyer = isContractBuyer(sellContract);
        // get token seller
        address tokenSeller = deferredContract.ownerOf(_tokenId);
        // check whether we need to send reward
        // we will send reward if the token has never been sold
        bool willSendReward = !soldTokens[_tokenId] &&
            sellContract.ekokeReward > 0;

        // get the currency token
        ERC20 currency = ERC20(usdErc20);

        // get the required allowance
        uint256 requiredAllowance = calcTokenPriceWithInterests(
            sellContract,
            _isContractBuyer
        );
        // check allowance on the currency token
        require(
            currency.allowance(msg.sender, address(this)) >= requiredAllowance,
            "Marketplace: Insufficient allowance"
        );
        // transfer USD from the `tokenBuyer` to the `tokenSeller`
        currency.transferFrom(
            tokenBuyer,
            tokenSeller,
            sellContract.tokenPriceUsd
        );
        // if the buyer is a contract buyer, transfer the interests to the marketplace
        if (_isContractBuyer) {
            currency.transferFrom(
                tokenBuyer,
                address(this),
                interests(sellContract.tokenPriceUsd)
            );
        }

        // if the buyer is a contract buyer, transfer the interest to the marketplace
        // transfer the NFT from the `tokenSeller` to the `tokenBuyer`
        ERC721(deferred).transferFrom(tokenSeller, tokenBuyer, _tokenId);

        // if we need to send reward, send the reward to the `tokenBuyer`
        if (willSendReward) {
            RewardPool(rewardPool).sendReward(
                tokenBuyer,
                sellContract.ekokeReward
            );
        }

        // set the token as sold
        soldTokens[_tokenId] = true;

        // emit the event
        emit TokenBought(
            tokenBuyer,
            tokenSeller,
            _tokenId,
            sellContract.tokenPriceUsd,
            requiredAllowance
        );
    }

    /// @notice Get the price of the token for the caller with the interests if the caller is a contract buyer
    /// @param _tokenId The ID of the deferred NFT
    /// @return _price The price of the token
    function tokenPriceForCaller(
        uint256 _tokenId
    ) external view returns (uint256) {
        Deferred deferredContract = Deferred(deferred);
        Deferred.SellContract memory sellContract = deferredContract
            .tokenContract(_tokenId);

        bool _isContractBuyer = isContractBuyer(sellContract);

        return calcTokenPriceWithInterests(sellContract, _isContractBuyer);
    }

    /// @notice Get whether the caller is a contract buyer
    /// @param _contract The contract to check
    /// @return _isContractBuyer Whether the caller is a contract buyer
    function isContractBuyer(
        Deferred.SellContract memory _contract
    ) internal view returns (bool) {
        for (uint256 i = 0; i < _contract.buyers.length; i++) {
            if (_contract.buyers[i] == msg.sender) {
                return true;
            }
        }

        return false;
    }

    /// @notice Calculate the price of the token with the interests if the buyer is a contract buyer
    /// @param _contract The contract to calculate the price for
    /// @param _isContractBuyer Whether the buyer is a contract buyer
    /// @return _price The price of the token
    function calcTokenPriceWithInterests(
        Deferred.SellContract memory _contract,
        bool _isContractBuyer
    ) internal view returns (uint256) {
        return
            _isContractBuyer
                ? _contract.tokenPriceUsd + interests(_contract.tokenPriceUsd)
                : _contract.tokenPriceUsd;
    }

    /// @notice Get the interests to pay for the token
    /// @param _usdPrice The price of the token in USD
    /// @return _interests The interests to pay
    function interests(
        uint256 _usdPrice
    ) internal view returns (uint256 _interests) {
        return (_usdPrice * interestRate) / 100;
    }
}
