import { expect } from "chai";
import { ethers } from "hardhat";
import { RewardPool, Ekoke } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

describe("RewardPool", () => {
  interface Contract {
    rewardPool: RewardPool;
    ekoke: Ekoke;
    owner: SignerWithAddress;
    marketplace: SignerWithAddress;
    deferred: SignerWithAddress;
    alice: SignerWithAddress;
  }

  let deploy: Contract;

  beforeEach(async () => {
    const [owner, marketplace, deferred, alice] = await ethers.getSigners();

    const ekokeContract = await ethers.deployContract("Ekoke", [owner.address]);
    const ekoke = ekokeContract as unknown as Ekoke;

    const rewardPoolContract = await ethers.deployContract("RewardPool", [
      owner.address,
      ekoke.getAddress(),
      deferred.address,
    ]);
    const rewardPool = rewardPoolContract as unknown as RewardPool;

    // set contracts
    await rewardPool.adminSetMarketplace(marketplace.address);
    await ekoke.adminSetRewardPoolAddress(rewardPool.getAddress());

    deploy = {
      rewardPool,
      ekoke,
      owner,
      marketplace,
      deferred,
      alice,
    };
  });

  it("should set marketplace", async () => {
    const { rewardPool, alice } = deploy;

    await rewardPool.adminSetMarketplace(alice.address);
  });

  it("Should reserve reward pool", async () => {
    const { rewardPool, deferred } = deploy;

    await rewardPool.connect(deferred).reservePool(10_000, 1000);

    // check reserved pool
    const reservedPool = await rewardPool.reservedAmount();

    expect(reservedPool).to.equal(10_000 * 1000);
  });

  it("Should not reserve reward pool if we already reached the limit", async () => {
    const { rewardPool, deferred, ekoke } = deploy;

    const maximumReward = await ekoke.MAX_REWARD_POOL_MINT();

    await expect(
      rewardPool.connect(deferred).reservePool(maximumReward, 2)
    ).to.be.revertedWith("RewardPool: reward pool has not enough liquidity");
  });

  it("Should not reserve reward pool if the caller is not deferred", async () => {
    const { rewardPool, ekoke } = deploy;

    const maximumReward = await ekoke.MAX_REWARD_POOL_MINT();

    await expect(rewardPool.reservePool(maximumReward, 2)).to.be.revertedWith(
      "RewardPool: caller is not deferred"
    );
  });

  it("Should send reward", async () => {
    const { rewardPool, deferred, marketplace, alice, ekoke } = deploy;

    const reward = 10_000;
    const totalReserved = 1000 * reward;
    await rewardPool.connect(deferred).reservePool(reward, 1000);

    await rewardPool.connect(marketplace).sendReward(alice.address, reward);

    // check alice balance
    expect(await ekoke.balanceOf(alice.address)).to.equal(reward);
    // check minted reward
    expect(await ekoke.totalSupply()).to.equal(reward);
    expect(await ekoke.rewardPoolMintedSupply()).to.equal(reward);
    // check reserved pool
    expect(await rewardPool.reservedAmount()).to.equal(totalReserved - reward);
  });

  it("Should not reward if not marketplace", async () => {
    const { rewardPool, deferred, marketplace, alice, ekoke } = deploy;

    const reward = 10_000;
    await rewardPool.connect(deferred).reservePool(reward, 1000);

    await expect(
      rewardPool.sendReward(alice.address, reward)
    ).to.be.revertedWith("RewardPool: caller is not the marketplace");
  });

  it("Should not reward if not enough liquidity", async () => {
    const { rewardPool, deferred, marketplace, alice } = deploy;

    const reward = 10_000;
    const totalReward = 1000 * reward;
    await rewardPool.connect(deferred).reservePool(reward, 1000);

    await expect(
      rewardPool.connect(marketplace).sendReward(alice.address, totalReward * 2)
    ).to.be.revertedWith("RewardPool: not enough reserved amount");
  });

  it("Should tell available rewards", async () => {
    const { rewardPool, deferred, ekoke, alice } = deploy;

    await rewardPool.connect(deferred).reservePool(10_000, 1000);
    const reserved = await rewardPool.reservedAmount();

    // trick to change temporarily the reward pool address
    await ekoke.adminSetRewardPoolAddress(alice.address);

    // mint some rewards on ekoke
    await ekoke.connect(alice).mintRewardTokens(alice.address, 100_000_000);

    // trick to change temporarily the reward pool address
    await ekoke.adminSetRewardPoolAddress(rewardPool.getAddress());

    const expectedAvailable =
      (await ekoke.MAX_REWARD_POOL_MINT()) - reserved - BigInt(100_000_000);

    // check available
    const availableReward = await rewardPool.availableReward();

    expect(availableReward).to.equal(expectedAvailable);
  });
});
