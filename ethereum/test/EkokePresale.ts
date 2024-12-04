import { expect } from "chai";
import { ethers } from "hardhat";
import { Ekoke, EkokePresale, TestERC20 } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const PRESALE_CAP = 10_000_000_000_000; // 100_000 EKOKE
const STEP_TOKENS = 100_000_000_000;
const STEP_TOKENS_WNO_DECIMALS = 1_000;
const BASE_TOKEN_PRICE = 1_000_000; // 1USDT
const SOFT_CAP = 2_000_000_000_000; // 20_000 EKOKE
const SOFT_CAP_WNO_DECIMALS = 20_000;
const USDT_DECIMALS = 6;

const usdToUsdt = (usd: number) => usd * 10 ** USDT_DECIMALS;
const ekokeToE8s = (ekoke: number) => ekoke * 10 ** 8;

describe("EkokePresale", () => {
  interface Contract {
    ekoke: Ekoke;
    usdt: TestERC20;
    presale: EkokePresale;
    owner: SignerWithAddress;
    alice: SignerWithAddress;
  }

  let deploy: Contract;

  beforeEach(async () => {
    const [owner, otherAccount] = await ethers.getSigners();
    const ekokeContract = await ethers.deployContract("Ekoke", [owner.address]);

    // send `PRESALE_CAP` to the presale contract
    const ekoke = ekokeContract as unknown as Ekoke;

    const usdtContract = await ethers.deployContract("TestERC20", [
      "USDT",
      "USDT",
      6,
    ]);

    const usdt = usdtContract as unknown as TestERC20;

    // mint 100_000 USDT to alice and owner
    await usdt.mint(owner.address, usdToUsdt(100_000));
    await usdt.mint(otherAccount.address, usdToUsdt(100_000));

    const ekokePresaleContract = await ethers.deployContract("EkokePresale", [
      owner.address,
      ekokeContract.getAddress(),
      usdtContract.getAddress(),
    ]);

    // mint cap to token and set presale cap
    const presale = ekokePresaleContract as unknown as EkokePresale;
    await ekoke.adminMint(ekokePresaleContract.getAddress(), PRESALE_CAP);
    await presale.adminSetPresaleCap();

    deploy = {
      ekoke,
      usdt,
      presale,
      owner,
      alice: otherAccount,
    };
  });

  it("Should have cap ", async () => {
    const { presale } = deploy;

    expect(await presale.presaleCap()).to.equal(PRESALE_CAP);
  });

  it("Should buy tokens", async () => {
    const { presale, owner, usdt } = deploy;

    const tokenPrice = await presale.tokenPrice();
    const totalPrice = tokenPrice * BigInt(1_000);

    // approve USDT
    await usdt.approve(presale.getAddress(), totalPrice);
    // call with value
    await presale.buyTokens(1_000);

    // verify balances
    expect(await presale.balanceOf(owner.address)).to.equal(ekokeToE8s(1_000));
    expect(await presale.usdInvested(owner.address)).to.equal(totalPrice);

    // check USDT balance
    expect(await usdt.balanceOf(owner.address)).to.equal(
      BigInt(usdToUsdt(100_000)) - totalPrice
    );
  });

  it("Should not buy tokens if we don't have USDT", async () => {
    const { presale, owner } = deploy;

    // call with value
    await expect(presale.buyTokens(1_000)).to.be.rejectedWith(Error);

    // verify balances
    expect(await presale.balanceOf(owner.address)).to.equal(0);
  });

  it("Should buy tokens twice", async () => {
    const { presale, owner, usdt } = deploy;

    const tokenPrice = await presale.tokenPrice();
    const totalPrice = tokenPrice * BigInt(100);

    // approve twice
    await usdt.approve(presale.getAddress(), totalPrice);

    // call with value
    await presale.buyTokens(100);
    await usdt.approve(presale.getAddress(), totalPrice);
    await presale.buyTokens(100);

    // verify balances
    expect(await presale.balanceOf(owner.address)).to.equal(ekokeToE8s(200));
    expect(await presale.usdInvested(owner.address)).to.equal(
      totalPrice * BigInt(2)
    );
  });

  it("Should get token price after step", async () => {
    const { presale, usdt } = deploy;

    const tokenPrice = await presale.tokenPrice();
    expect(tokenPrice).to.equal(BASE_TOKEN_PRICE);

    // buy tokens to reach the step
    const tokensToBuy = STEP_TOKENS_WNO_DECIMALS;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);
    await usdt.approve(presale.getAddress(), totalPrice);
    await presale.buyTokens(tokensToBuy);

    // check new price
    const newTokenPrice = await presale.tokenPrice();
    await usdt.approve(presale.getAddress(), newTokenPrice);
    expect(newTokenPrice).to.equal(BASE_TOKEN_PRICE * 2);
  });

  it("Should claim tokens after presale is closed", async () => {
    const { presale, ekoke, usdt, alice } = deploy;

    // buy soft cap tokens, so we succeed
    const tokenPrice = await presale.tokenPrice();
    const tokensToBuy = SOFT_CAP_WNO_DECIMALS;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);

    await usdt.connect(alice).approve(presale.getAddress(), totalPrice);
    await presale.connect(alice).buyTokens(tokensToBuy);

    // close presale
    await presale.adminClosePresale();

    // claim tokens
    await presale.connect(alice).claimTokens();

    // get my ekoke balance
    const balance = await ekoke.balanceOf(alice.address);

    // verify balances
    expect(balance).to.equal(SOFT_CAP);

    // verify presale balance is 0
    expect(await presale.balanceOf(alice.address)).to.equal(0);

    // verify we can't get refunded
    await expect(presale.refund()).to.be.revertedWith(
      "EkokePresale: Presale did not fail"
    );

    // verify we can't claim tokens again
    await expect(presale.claimTokens()).to.be.revertedWith(
      "EkokePresale: No tokens to claim"
    );
  });

  it("Should send USDT raised to owner after close", async () => {
    const { presale, ekoke, owner, alice, usdt } = deploy;

    // buy soft cap tokens, so we succeed
    const tokenPrice = await presale.tokenPrice();
    const tokensToBuy = SOFT_CAP_WNO_DECIMALS;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);

    await usdt.connect(alice).approve(presale.getAddress(), totalPrice);
    await presale.connect(alice).buyTokens(tokensToBuy);

    // close presale
    const previousBalance = await usdt.balanceOf(owner.address);
    await presale.adminClosePresale();
    expect(await presale.isOpen()).to.be.false;

    // check owner ETH balance
    const ownerBalanceDiff =
      (await usdt.balanceOf(owner.address)) - previousBalance;
    expect(ownerBalanceDiff).to.be.equal(totalPrice);

    // check owner got the remaining tokens
    const remainingEkoke = PRESALE_CAP - SOFT_CAP;
    const ownerEkokeBalance = await ekoke.balanceOf(owner.address);
    expect(ownerEkokeBalance).to.equal(remainingEkoke);
  });

  it("Should refund tokens after presale is failed", async () => {
    const { presale, alice, usdt } = deploy;

    // buy soft cap tokens, so we succeed
    const tokenPrice = await presale.tokenPrice();
    const tokensToBuy = 100;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);

    await usdt.connect(alice).approve(presale.getAddress(), totalPrice);
    await presale.connect(alice).buyTokens(tokensToBuy);

    // close presale
    await presale.adminClosePresale();

    expect(await presale.isOpen()).to.be.false;

    // claim should fail
    await expect(presale.claimTokens()).to.be.revertedWith(
      "EkokePresale: Presale failed"
    );

    const usdtBalanceBefore = await usdt.balanceOf(alice.address);

    // refund
    await presale.connect(alice).refund();

    // check alice ETH balance
    const usdBalanceDiff =
      (await usdt.balanceOf(alice.address)) - usdtBalanceBefore;
    expect(usdBalanceDiff).to.be.equal(totalPrice);

    // should not allow to buy
    await expect(presale.buyTokens(1_000)).to.be.revertedWith(
      "EkokePresale: Presale is closed"
    );
  });
});
