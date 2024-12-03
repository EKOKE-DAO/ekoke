import { expect } from "chai";
import { ethers } from "hardhat";
import { Ekoke, EkokePresale } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const PRESALE_CAP = 10_000_000_000_000; // 100_000 EKOKE
const STEP_TOKENS = 100_000_000_000;
const BASE_TOKEN_PRICE = 1_000_000_000;
const SOFT_CAP = 2_000_000_000_000; // 20_000 EKOKE

describe("EkokePresale", () => {
  interface Contract {
    ekoke: Ekoke;
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

    const ekokePresaleContract = await ethers.deployContract("EkokePresale", [
      owner.address,
      ekokeContract.getAddress(),
    ]);

    // mint cap to token and set presale cap
    const presale = ekokePresaleContract as unknown as EkokePresale;
    await ekoke.adminMint(ekokePresaleContract.getAddress(), PRESALE_CAP);
    await presale.adminSetPresaleCap();

    deploy = {
      ekoke,
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
    const { presale, owner } = deploy;

    const tokenPrice = await presale.tokenPrice();
    const totalPrice = tokenPrice * BigInt(1_000);

    // call with value
    await presale.buyTokens(1_000, { value: totalPrice });

    // verify balances
    expect(await presale.balanceOf(owner.address)).to.equal(1_000);
  });

  it("Should not buy tokens if we don't have eth", async () => {
    const { presale, owner } = deploy;

    const tokenPrice = await presale.tokenPrice();

    // call with value
    await expect(
      presale.buyTokens(1_000, { value: tokenPrice })
    ).to.be.revertedWith("EkokePresale: Not enough ETH to buy tokens");

    // verify balances
    expect(await presale.balanceOf(owner.address)).to.equal(0);
  });

  it("Should buy tokens twice", async () => {
    const { presale, owner } = deploy;

    const tokenPrice = await presale.tokenPrice();
    const totalPrice = tokenPrice * BigInt(1_000);

    // call with value
    await presale.buyTokens(1_000, { value: totalPrice });
    await presale.buyTokens(1_000, { value: totalPrice });

    // verify balances
    expect(await presale.balanceOf(owner.address)).to.equal(2_000);
  });

  it("Should get token price after step", async () => {
    const { presale } = deploy;

    const tokenPrice = await presale.tokenPrice();
    expect(tokenPrice).to.equal(BASE_TOKEN_PRICE);

    // buy tokens to reach the step
    const tokensToBuy = STEP_TOKENS;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);
    await presale.buyTokens(tokensToBuy, { value: totalPrice });

    // check new price
    const newTokenPrice = await presale.tokenPrice();
    expect(newTokenPrice).to.equal(BASE_TOKEN_PRICE * 2);
  });

  it("Should claim tokens after presale is closed", async () => {
    const { presale, ekoke, owner, alice } = deploy;

    // buy soft cap tokens, so we succeed
    const tokenPrice = await presale.tokenPrice();
    const tokensToBuy = SOFT_CAP;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);

    await presale.connect(alice).buyTokens(tokensToBuy, { value: totalPrice });

    // close presale
    await presale.adminClosePresale();

    // claim tokens
    await presale.connect(alice).claimTokens();

    // get my ekoke balance
    const balance = await ekoke.balanceOf(alice.address);

    // verify balances
    expect(balance).to.equal(tokensToBuy);

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

  it("Should send ETH raised to owner after close", async () => {
    const { presale, ekoke, owner, alice } = deploy;

    // buy soft cap tokens, so we succeed
    const tokenPrice = await presale.tokenPrice();
    const tokensToBuy = SOFT_CAP;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);

    await presale.connect(alice).buyTokens(tokensToBuy, { value: totalPrice });

    // close presale
    const previousBalance = await ethers.provider.getBalance(owner.address);
    await presale.adminClosePresale();
    expect(await presale.isOpen()).to.be.false;

    // check owner ETH balance
    const ownerBalanceDiff =
      (await ethers.provider.getBalance(owner.address)) - previousBalance;
    expect(ownerBalanceDiff).to.be.gt(0);

    // check owner got the remaining tokens
    const remainingEkoke = PRESALE_CAP - SOFT_CAP;
    const ownerEkokeBalance = await ekoke.balanceOf(owner.address);
    expect(ownerEkokeBalance).to.equal(remainingEkoke);
  });

  it("Should refund tokens after presale is failed", async () => {
    const { presale, alice } = deploy;

    // buy soft cap tokens, so we succeed
    const tokenPrice = await presale.tokenPrice();
    const tokensToBuy = 100_000;
    const totalPrice = tokenPrice * BigInt(tokensToBuy);

    await presale.connect(alice).buyTokens(tokensToBuy, { value: totalPrice });

    // close presale
    await presale.adminClosePresale();

    expect(await presale.isOpen()).to.be.false;

    // claim should fail
    await expect(presale.claimTokens()).to.be.revertedWith(
      "EkokePresale: Presale failed"
    );

    const ethBalanceBefore = await ethers.provider.getBalance(alice.address);

    // refund
    await presale.connect(alice).refund();

    // check alice ETH balance
    const ethBalanceDiff =
      (await ethers.provider.getBalance(alice.address)) - ethBalanceBefore;
    expect(ethBalanceDiff).to.be.gt(0);

    // should not allow to buy
    await expect(
      presale.buyTokens(1_000, { value: totalPrice })
    ).to.be.revertedWith("EkokePresale: Presale is closed");
  });
});
