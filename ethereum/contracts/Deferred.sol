// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {RewardPool} from "./RewardPool.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Deferred is ERC721, Ownable {
    struct SellerRequest {
        /// @dev The address of the seller
        address seller;
        /// @dev The amount of tokens the seller has
        uint8 quota;
    }

    /// @dev Data to create a contract
    struct CreateContractRequest {
        /// @dev metadata uri pointing to deferred-data canister uri
        string metadataUri;
        /// @dev Contract sellers
        SellerRequest[] sellers;
        /// @dev The contract buyers
        address[] buyers;
        /// @dev Reward for buying a token
        uint256 ekokeReward;
        /// @dev The price of the token in USD
        uint256 tokenPriceUsd;
        /// @dev the amount of tokens to mint
        uint256 tokensAmount;
    }

    /// @dev Seller data
    struct Seller {
        /// @dev The address of the seller
        address seller;
        /// @dev Token from id associated to the seller
        uint256 tokenFromId;
        /// @dev Token to id associated to the seller
        uint256 tokenToId;
    }

    /// @dev Sell contract of a real estate token
    struct SellContract {
        /// @dev metadata uri pointing to deferred-data canister uri
        string metadataUri;
        /// @dev Contract sellers
        Seller[] sellers;
        /// @dev The contract buyers
        address[] buyers;
        /// @dev Reward for buying a token
        uint256 ekokeReward;
        /// @dev The price of the token in USD
        uint256 tokenPriceUsd;
        /// @dev The first id of a token associated to the contract (lazy minting)
        uint256 tokenFromId;
        /// @dev The last id of a token associated to the contract (lazy minting)
        uint256 tokenToId;
    }

    /// @dev The sell contracts
    mapping(uint256 => SellContract) private sellContracts;

    /// @dev lazy balances
    mapping(address => uint256) private lazyBalances;

    /// @dev The next sell contract id. ZERO is reserved for non-existing contracts
    uint256 private nextSellContractId = 1;

    /// @dev The next token id
    uint256 private nextTokenId = 0;

    /// @dev Marketplace address
    address public marketplace = address(0);

    /// @dev Reward pool address
    address public rewardPool = address(0);

    /// @dev The deferred minter address
    address public deferredMinter = address(0);

    modifier onlyMinter() {
        require(
            msg.sender == deferredMinter && deferredMinter != address(0),
            "Deferred: caller is not the minter"
        );
        _;
    }

    constructor(
        address _initialOwner
    ) ERC721("Deferred", "DEFERRED") Ownable(_initialOwner) {}

    /// @notice Set the marketplace address
    /// @param _marketplace The marketplace address
    function adminSetMarketplace(address _marketplace) external onlyOwner {
        marketplace = _marketplace;
    }

    /// @notice Set the reward pool address
    /// @param _rewardPool The reward pool address
    function adminSetRewardPool(address _rewardPool) external onlyOwner {
        rewardPool = _rewardPool;
    }

    /// @notice Set the deferred minter address
    /// @param _deferredMinter The deferred minter address
    function adminSetDeferredMinter(
        address _deferredMinter
    ) external onlyOwner {
        deferredMinter = _deferredMinter;
    }

    /// @notice Create a sell contract. Only the minter can call this method
    /// @param _request The request to create a contract
    /// @return sellContractId The id of the created contract
    function createContract(
        CreateContractRequest memory _request
    ) external onlyMinter returns (uint256 sellContractId) {
        require(rewardPool != address(0), "Deferred: reward pool is not set");
        require(
            _request.tokensAmount > 0,
            "Deferred: tokensAmount must be greater than 0"
        );
        uint8 totalQuota = 0;
        for (uint256 i = 0; i < _request.sellers.length; i++) {
            require(
                _request.sellers[i].seller != address(0),
                "Deferred: seller must be set"
            );
            totalQuota += _request.sellers[i].quota;
        }
        require(totalQuota == 100, "Deferred: total quota must be 100");
        require(
            _request.tokenPriceUsd > 0,
            "Deferred: tokenPriceUsd must be greater than 0"
        );
        require(
            _request.tokensAmount % 100 == 0,
            "Deferred: tokensAmount must be divisible by 100"
        );

        // reserve pool on reward pool
        if (_request.ekokeReward > 0) {
            RewardPool(rewardPool).reservePool(
                _request.ekokeReward,
                _request.tokensAmount
            );
        }

        uint256 tokenFromId = nextTokenId;
        uint256 tokenToId = tokenFromId + _request.tokensAmount - 1;
        uint256 contractId = nextSellContractId;

        // make sellers array
        Seller[] memory sellers = new Seller[](_request.sellers.length);
        uint256 tokenFromForSeller = tokenFromId;
        uint256 tokensForQuota = _request.tokensAmount / 100;

        // create sellers with their token range
        for (uint256 i = 0; i < _request.sellers.length; i++) {
            // Get token range by quota
            SellerRequest memory sellerRequest = _request.sellers[i];
            uint256 tokensForSeller = tokensForQuota * sellerRequest.quota;
            uint256 tokenToForSeller = tokenFromForSeller + tokensForSeller - 1;

            sellers[i] = Seller({
                seller: _request.sellers[i].seller,
                tokenFromId: tokenFromForSeller,
                tokenToId: tokenToForSeller
            });
            tokenFromForSeller = tokenToForSeller + 1;
            // increment lazy balance
            lazyBalances[sellers[i].seller] += tokensForSeller;
        }

        SellContract storage sellContract = sellContracts[contractId];
        sellContract.metadataUri = _request.metadataUri;
        sellContract.buyers = _request.buyers;
        sellContract.ekokeReward = _request.ekokeReward;
        sellContract.tokenPriceUsd = _request.tokenPriceUsd;
        sellContract.tokenFromId = tokenFromId;
        sellContract.tokenToId = tokenToId;
        // push sellers
        delete sellContract.sellers;
        for (uint256 i = 0; i < sellers.length; i++) {
            sellContract.sellers.push(sellers[i]);
        }

        nextTokenId += _request.tokensAmount;
        nextSellContractId += 1;

        return contractId;
    }

    // ERC721 overrides

    /// @notice Get balance of a token owner
    /// @param owner The address of the token owner
    /// @return balance The balance of the token owner
    function balanceOf(
        address owner
    ) public view override returns (uint256 balance) {
        return super.balanceOf(owner) + lazyBalances[owner];
    }

    /// @notice Get the owner of a token
    /// @param tokenId The id of the token
    /// @return owner The address of the token owner
    function ownerOf(
        uint256 tokenId
    ) public view override returns (address owner) {
        if (_isLazy(tokenId)) {
            return _initialTokenOwner(_tokenContract(tokenId), tokenId);
        }
        return super.ownerOf(tokenId);
    }

    /// @notice Transfer a token from an address to another
    /// @dev This method is called by the marketplace
    /// @param from The address of the token owner
    /// @param to The address of the token receiver
    /// @param tokenId The id of the token
    /// @param data idk
    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId,
        bytes memory data
    ) public override {
        require(
            msg.sender == marketplace && marketplace != address(0),
            "Deferred: caller is not the marketplace"
        );

        // if is lazy minting, mint the token
        if (_isLazy(tokenId)) {
            _lazyMint(tokenId, to);
            lazyBalances[from] -= 1;

            // approve the marketplace to transfer the token
            super.approve(marketplace, tokenId);

            return;
        }

        // if the token is not allowed to marketplace, approve it first
        if (super.getApproved(tokenId) != marketplace) {
            super.approve(marketplace, tokenId);
        }

        return super.safeTransferFrom(from, to, tokenId, data);
    }

    /// @notice Get the token uri
    /// @param tokenId The id of the token
    /// @return metadataUri The metadata uri of the token
    function tokenURI(
        uint256 tokenId
    ) public view override returns (string memory) {
        return tokenContract(tokenId).metadataUri;
    }

    /// @notice Get the token price in USD
    /// @param tokenId The id of the token
    /// @return tokenPriceUsd The price of the token in USD
    function tokenPriceUsd(uint256 tokenId) public view returns (uint256) {
        return tokenContract(tokenId).tokenPriceUsd;
    }

    /// @notice Get the token contract
    /// @param tokenId The id of the token
    /// @return sellContract The contract of the token
    function tokenContract(
        uint256 tokenId
    ) public view returns (SellContract memory) {
        uint256 contractId = _tokenContract(tokenId);
        require(contractId > 0, "Deferred: token does not exist");

        return sellContracts[contractId];
    }

    /// @notice Transfer a token from an address to another
    /// @dev This method is called by the marketplace
    /// @param from The address of the token owner
    /// @param to The address of the token receiver
    /// @param tokenId The id of the token
    function transferFrom(
        address from,
        address to,
        uint256 tokenId
    ) public override {
        require(
            msg.sender == marketplace && marketplace != address(0),
            "Deferred: caller is not the marketplace"
        );

        // if is lazy minting, mint the token
        if (_isLazy(tokenId)) {
            _lazyMint(tokenId, to);
            lazyBalances[from] -= 1;

            // approve the marketplace to transfer the token
            super.approve(marketplace, tokenId);

            return;
        }

        // if the token is not allowed to marketplace, approve it first
        if (super.getApproved(tokenId) != marketplace) {
            super.approve(marketplace, tokenId);
        }

        super.transferFrom(from, to, tokenId);
    }

    /// @notice Approve a token to be transferred to another address
    /// @dev This method is not allowed. Only the marketplace can approve a token
    function approve(address, uint256) public pure override {
        revert("Deferred: approve is not allowed");
    }

    /// @notice Approve all tokens to be transferred to another address
    /// @dev This method is not allowed. Only the marketplace can approve all tokens
    function setApprovalForAll(address, bool) public pure override {
        revert("Deferred: setApprovalForAll is not allowed");
    }

    /// @notice Get the address that approved a token to be transferred to another address
    /// @param tokenId The id of the token
    /// @return approvedAddress The address that approved the token
    function getApproved(
        uint256 tokenId
    ) public view override returns (address) {
        if (_isLazy(tokenId)) {
            return marketplace;
        }

        return super.getApproved(tokenId);
    }

    /// @notice Check if an address is approved to transfer all tokens of another address
    /// @param owner The address of the token owner
    /// @param operator The address of the operator
    function isApprovedForAll(
        address owner,
        address operator
    ) public view override returns (bool) {
        return operator == marketplace || operator == owner;
    }

    // privates

    /// @notice Given a contract id and the token id, returns the address of the seller that should initially own the token based on its quota
    /// @param _contractId The id of the contract
    /// @param _tokenId The id of the token
    /// @return _initialOwner The address of the initial owner
    function _initialTokenOwner(
        uint256 _contractId,
        uint256 _tokenId
    ) internal view returns (address _initialOwner) {
        require(_contractId > 0, "Deferred: contractId must be greater than 0");
        require(
            _contractId < nextSellContractId,
            "Deferred: contract does not exist"
        );
        require(
            _tokenId >= sellContracts[_contractId].tokenFromId,
            "Deferred: tokenId must be greater than or equal to tokenFromId"
        );
        require(
            _tokenId <= sellContracts[_contractId].tokenToId,
            "Deferred: tokenId must be less than or equal to tokenToId"
        );
        SellContract memory sellContract = sellContracts[_contractId];
        // get the quota the token belongs to
        for (uint256 i = 0; i < sellContract.sellers.length; i++) {
            Seller memory seller = sellContract.sellers[i];
            if (
                seller.tokenFromId <= _tokenId && _tokenId <= seller.tokenToId
            ) {
                return seller.seller;
            }
        }

        // raise an error if the token does not belong to any seller
        revert("Deferred: token does not belong to any seller");
    }

    /// @notice lazy mint a token
    /// @param _tokenId The id of the token
    /// @param _to The address of the buyer
    function _lazyMint(uint256 _tokenId, address _to) internal {
        _safeMint(_to, _tokenId);
    }

    /// @notice returns whether a token is in lazy minting state
    /// @param _tokenId The id of the token
    /// @return isLazy True if the token is in lazy minting state
    function _isLazy(uint256 _tokenId) internal view returns (bool isLazy) {
        try this._inheritedOwnerOf(_tokenId) returns (address) {
            return false;
        } catch {
            return true;
        }
    }

    /// @notice Workaround to be able to call the ownerOf method from the parent contract
    /// @dev workaround for try-catching the ownerOf method
    function _inheritedOwnerOf(uint256 tokenId) public view returns (address) {
        return super.ownerOf(tokenId);
    }

    /// @notice Returns the contract id of a token
    /// @param _tokenId The id of the token
    /// @return contractId The id of the contract; 0 if the token is not in lazy minting state
    function _tokenContract(
        uint256 _tokenId
    ) internal view returns (uint256 contractId) {
        // check if the token is in a sell contract
        for (uint256 i = 1; i < nextSellContractId; i++) {
            SellContract memory sellContract = sellContracts[i];
            if (
                sellContract.tokenFromId <= _tokenId &&
                _tokenId <= sellContract.tokenToId
            ) {
                return i;
            }
        }

        return 0;
    }
}
