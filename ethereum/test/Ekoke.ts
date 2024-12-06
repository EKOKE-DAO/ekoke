import { expect } from "chai";
import { ethers } from "hardhat";
import { Ekoke } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const NAME = "Ekoke";
const SYMBOL = "EKOKE";
const DECIMALS = 9;

describe("Ekoke", () => {
  interface Contract {
    token: Ekoke;
    owner: SignerWithAddress;
    rewardPool: SignerWithAddress;
  }

  let deploy: Contract;

  beforeEach(async () => {
    const [owner, otherAccount] = await ethers.getSigners();
    const contract = await ethers.deployContract(NAME, [owner.address]);

    deploy = {
      token: contract as unknown as Ekoke,
      owner,
      rewardPool: otherAccount,
    };
  });

  it("Should has the correct name and symbol ", async () => {
    const { token, owner } = deploy;
    expect(await token.name()).to.equal(NAME);
    expect(await token.symbol()).to.equal(SYMBOL);
    expect(await token.decimals()).to.equal(DECIMALS);
    // check balance
    expect(await token.balanceOf(owner.address)).to.equal(0);
    // check reward pool is zero
    expect(await token.rewardPool()).to.equal(
      "0x0000000000000000000000000000000000000000"
    );
  });

  it("Should set reward pool address", async () => {
    const { token, rewardPool } = deploy;
    await token.adminSetRewardPoolAddress(rewardPool.address);
    expect(await token.rewardPool()).to.equal(rewardPool.address);
  });

  it("Should transfer 500 tokens", async () => {
    const { rewardPool, owner, token } = deploy;
    await token.adminMint(owner.address, 1_000);

    await token.transfer(rewardPool.address, 250);
    expect(await token.balanceOf(rewardPool.address)).to.equal(250);
    expect(await token.balanceOf(owner.address)).to.equal(750);
  });

  it("should renounce ownership", async () => {
    const { token } = deploy;
    await token.renounceOwnership();
    expect(await token.owner()).to.equal(
      "0x0000000000000000000000000000000000000000"
    );
  });

  it("should return supply", async () => {
    const { token, owner } = deploy;
    await token.adminMint(owner.address, 1_000);
    expect(await token.totalSupply()).to.equal(1_000);
  });

  it("should mint owner tokens", async () => {
    const { token, owner } = deploy;
    await token.adminMint(owner.address, 1_000);
    expect(await token.balanceOf(owner.address)).to.equal(1_000);
    expect(await token.ownerMintedSupply()).to.equal(1_000);
  });

  it("should not allow minting too many owner tokens", async () => {
    const { token, owner } = deploy;
    const maxOwnerSupply = await token.MAX_OWNER_MINT();
    await token.adminMint(owner.address, maxOwnerSupply);

    expect(await token.balanceOf(owner.address)).to.equal(maxOwnerSupply);
    expect(await token.ownerMintedSupply()).to.equal(maxOwnerSupply);

    // try to mint one more token
    await expect(token.adminMint(owner.address, 1)).to.be.revertedWith(
      "Ekoke: owner minting limit reached"
    );
  });

  it("should not allow mint from non owner", async () => {
    const { rewardPool, token } = deploy;
    await expect(
      token.connect(rewardPool).adminMint(rewardPool.address, 1000)
    ).to.be.rejectedWith(Error);
  });

  it("should mint reward tokens", async () => {
    const { token, owner, rewardPool } = deploy;

    await token.adminSetRewardPoolAddress(rewardPool.address);

    await token.connect(rewardPool).mintRewardTokens(owner.address, 1_000);
    expect(await token.balanceOf(owner.address)).to.equal(1_000);
    expect(await token.rewardPoolMintedSupply()).to.equal(1_000);
  });

  it("should not allow minting too many reward tokens", async () => {
    const { token, rewardPool, owner } = deploy;
    const maxRewardSupply = await token.MAX_REWARD_POOL_MINT();

    await token.adminSetRewardPoolAddress(rewardPool.address);
    await token
      .connect(rewardPool)
      .mintRewardTokens(owner.address, maxRewardSupply);

    expect(await token.balanceOf(owner.address)).to.equal(maxRewardSupply);
    expect(await token.rewardPoolMintedSupply()).to.equal(maxRewardSupply);

    // try to mint one more token
    await expect(
      token.connect(rewardPool).mintRewardTokens(owner.address, 1)
    ).to.be.revertedWith("Ekoke: reward pool minting limit reached");
  });

  it("should not allow mint from non reward pool", async () => {
    const { owner, token } = deploy;
    await expect(
      token.connect(owner).mintRewardTokens(owner.address, 1000)
    ).to.be.rejectedWith("Ekoke: caller is not the reward pool");
  });

  it("should transfer ownership of the contract", async () => {
    const { rewardPool, token } = deploy;
    await token.transferOwnership(rewardPool.address);
    expect(await token.owner()).to.equal(rewardPool.address);
  });

  it("should burn tokens to allow more minting by the reward pool", async () => {
    const { token, rewardPool, owner } = deploy;
    const maxRewardSupply = await token.MAX_REWARD_POOL_MINT();

    await token.adminSetRewardPoolAddress(rewardPool.address);
    await token
      .connect(rewardPool)
      .mintRewardTokens(owner.address, maxRewardSupply);

    expect(await token.balanceOf(owner.address)).to.equal(maxRewardSupply);
    expect(await token.rewardPoolMintedSupply()).to.equal(maxRewardSupply);

    // burn some tokens
    const amount = 100_000;
    await token.burn(amount);

    // mint more tokens
    expect(await token.rewardPoolMintedSupply()).to.equal(
      maxRewardSupply - BigInt(amount)
    );
    expect(await token.balanceOf(owner.address)).to.equal(
      maxRewardSupply - BigInt(amount)
    );

    // should mint more tokens
    await token.connect(rewardPool).mintRewardTokens(owner.address, 100_000);
  });

  it("should not allowing burning if reward pool has not mint so much", async () => {
    const { token, owner } = deploy;

    await token.adminMint(owner.address, 1_000);

    await expect(token.burn(1000)).to.be.revertedWith(
      "Ekoke: amount exceeds the reward pool minted supply"
    );
  });
});
