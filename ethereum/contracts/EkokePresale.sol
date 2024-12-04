// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract EkokePresale is Ownable {
    /// @notice Base price of 1 EKOKE token in USDT (6 decimals)
    uint256 private baseTokenPrice = 1_000_000; // 1 USDT

    /// @notice The soft cap of the presale in EKOKE tokens
    uint256 public constant SOFT_CAP = 2_000_000_000_000; // 20_000 EKOKE

    /// @notice The step at which the token price is increased.
    /// @dev The token price is summed to baseTokenPrice every TOKEN_PRICE_STEP tokens sold
    uint256 private constant TOKEN_PRICE_STEP = 100_000_000_000; // 1_000 EKOKE

    /// @notice The address of the EKOKE token
    address public ekoke;

    /// @notice The address of the USDT token
    address public usdt;

    /// @notice The amount of EKOKE tokens reserved for the presale
    mapping(address => uint256) private presaleAmounts;

    /// @notice The amount of USD paid by an account (used for refunds)
    mapping(address => uint256) private usdPaid;

    /// @notice The amount of EKOKE tokens sold in the presale
    uint256 public tokensSold = 0;

    /// @notice The cap of the presale in EKOKE tokens
    uint256 public presaleCap = 0;

    /// @notice Whether the presale is open
    bool private presaleOpen = true;

    /// @notice Whether the presale failed
    bool private presaleFailed = false;

    /// @notice Event emitted when tokens are sold
    event TokensSold(address indexed buyer, uint256 amount);

    /// @notice Event emitted when tokens are claimed
    event TokensClaimed(address indexed buyer, uint256 amount);

    /// @notice Event emitted when tokens are refunded
    event TokensRefunded(address indexed buyer, uint256 amount);

    /// @notice Event emitted when the presale is closed
    event PresaleClosed();

    modifier onlyPresaleOpen() {
        require(presaleOpen, "EkokePresale: Presale is closed");
        _;
    }

    modifier onlyPresaleFailed() {
        require(
            presaleFailed && !presaleOpen,
            "EkokePresale: Presale did not fail"
        );
        _;
    }

    modifier onlyPresaleSucceeded() {
        require(!presaleFailed && !presaleOpen, "EkokePresale: Presale failed");
        _;
    }

    constructor(address _owner, address _ekoke, address _usdt) Ownable(_owner) {
        require(_ekoke != address(0), "EkokePresale: EKOKE address is zero");
        require(_usdt != address(0), "EkokePresale: USDT address is zero");
        ekoke = _ekoke;
        usdt = _usdt;
    }

    /// @notice Get whether the presale is open
    /// @return _isOpen Whether the presale is open
    function isOpen() public view returns (bool _isOpen) {
        return presaleOpen;
    }

    /// @notice Get whether the presale has failed
    /// @return _hasFailed Whether the presale has failed
    function hasFailed() public view returns (bool _hasFailed) {
        return presaleFailed;
    }

    /// @notice Get the current token price for 1$EKOKE in USDT. 1 token = 100_000_000 EKOKE with decimals
    /// @dev token price is baseTokenPrice * steps, where a step is TOKEN_PRICE_STEP tokens sold
    function tokenPrice() public view returns (uint256) {
        // get steps
        uint256 steps = (tokensSold / TOKEN_PRICE_STEP) + 1;

        // return the token price
        return baseTokenPrice * steps;
    }

    /// @notice Get the amount of EKOKE tokens bought by an account
    /// @param _account The account to get the balance of
    /// @return balance The amount of EKOKE tokens bought by the account
    function balanceOf(address _account) public view returns (uint256 balance) {
        return presaleAmounts[_account];
    }

    /// @notice Get the amount of USD invested by an account
    /// @param _account The account to get the balance of
    /// @return invested The amount of USD invested by the account
    function usdInvested(
        address _account
    ) public view returns (uint256 invested) {
        return usdPaid[_account];
    }

    /// @notice Buy presale tokens. The amount of tokens is 100_000_000 EKOKE with decimals, because you can't buy less than 1 token
    /// @param _amount The amount of tokens to buy
    function buyTokens(uint256 _amount) external onlyPresaleOpen {
        uint256 realAmount = _amount * 100_000_000;

        uint256 remainingPresaleBalance = presaleBalance();
        require(
            remainingPresaleBalance >= realAmount,
            "EkokePresale: Not enough tokens in the presale"
        );

        uint256 currentTokenPrice = tokenPrice();
        uint256 usdToPay = _amount * currentTokenPrice; // NOTE: price must be calculated against the integer amount

        // check allowance and transfer USDT
        IERC20(usdt).transferFrom(msg.sender, address(this), usdToPay);

        // set the amount of tokens bought by the account and increase the total amount of tokens sold
        presaleAmounts[msg.sender] += realAmount;
        tokensSold += realAmount;
        // increase the amount of USD paid by the account
        usdPaid[msg.sender] += usdToPay;

        emit TokensSold(msg.sender, realAmount);
    }

    /// @notice Claims the EKOKE tokens bought in the presale
    function claimTokens() external onlyPresaleSucceeded {
        uint256 amount = presaleAmounts[msg.sender];
        require(amount > 0, "EkokePresale: No tokens to claim");

        presaleAmounts[msg.sender] = 0;
        IERC20(ekoke).transfer(msg.sender, amount);

        emit TokensClaimed(msg.sender, amount);
    }

    /// @notice Refunds the EKOKE tokens bought in the presale in case of failure
    function refund() external onlyPresaleFailed {
        uint256 amount = presaleAmounts[msg.sender];
        require(amount > 0, "EkokePresale: No tokens to refund");
        uint256 refundAmount = usdPaid[msg.sender];
        require(refundAmount > 0, "EkokePresale: No USD to refund");

        presaleAmounts[msg.sender] = 0;
        usdPaid[msg.sender] = 0;
        IERC20(usdt).transfer(msg.sender, refundAmount);

        emit TokensRefunded(msg.sender, refundAmount);
    }

    /// @notice Close the presale. From now on, no more tokens can be bought. If the soft cap is not reached, the presale is considered failed
    function adminClosePresale() external onlyOwner onlyPresaleOpen {
        presaleOpen = false;
        if (tokensSold < SOFT_CAP) {
            presaleFailed = true;
        } else {
            presaleFailed = false;
            // in case of success, transfer USDT balance to the owner
            uint256 usdtBalance = IERC20(usdt).balanceOf(address(this));
            if (usdtBalance > 0) {
                IERC20(usdt).transfer(owner(), usdtBalance);
            }
            // send also unsold tokens to the owner
            uint256 remainingPresaleBalance = presaleCap - tokensSold;
            if (remainingPresaleBalance > 0) {
                IERC20(ekoke).transfer(owner(), remainingPresaleBalance);
            }
        }
    }

    /// @notice Set the presale cap to the current balance of the presale contract
    function adminSetPresaleCap() external onlyOwner {
        require(presaleCap == 0, "EkokePresale: Presale cap already set");

        presaleCap = IERC20(ekoke).balanceOf(address(this));
    }

    /// @notice Get the current $EKOKE balance of the presale contract
    /// @return balance The balance of the presale contract
    function presaleBalance() internal view returns (uint256 balance) {
        return IERC20(ekoke).balanceOf(address(this));
    }
}
