import { expect } from "chai";
import { ethers } from "hardhat";
import { Deferred, RewardPool, Ekoke } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const NAME = "Deferred";

describe("Deferred", () => {
  interface Contract {
    token: Deferred;
    rewardPoolContract: RewardPool;
    ekokeContract: Ekoke;
    owner: SignerWithAddress;
    marketplace: SignerWithAddress;
    minter: SignerWithAddress;
    alice: SignerWithAddress;
    bob: SignerWithAddress;
    charlie: SignerWithAddress;
  }

  let deploy: Contract;

  beforeEach(async () => {
    const [owner, marketplace, minter, alice, bob, charlie] =
      await ethers.getSigners();
    const contract = await ethers.deployContract(NAME, [owner.address]);
    // set contracts
    const token = contract as unknown as Deferred;

    const ekokeContract = await ethers.deployContract("Ekoke", [owner.address]);
    const ekoke = ekokeContract as unknown as Ekoke;

    const rewardPoolContract = await ethers.deployContract("RewardPool", [
      owner.address,
      ekoke.getAddress(),
      contract.getAddress(),
    ]);
    const rewardPool = rewardPoolContract as unknown as RewardPool;

    // set contracts
    await token.adminSetDeferredMinter(minter.address);
    await token.adminSetRewardPool(rewardPool.getAddress());
    await token.adminSetMarketplace(marketplace.address);

    deploy = {
      token,
      rewardPoolContract: rewardPool,
      ekokeContract: ekoke,
      owner,
      marketplace,
      minter,
      alice,
      bob,
      charlie,
    };
  });

  it("Should has the correct name and symbol ", async () => {
    const { token, marketplace } = deploy;
    expect(await token.name()).to.equal(NAME);
    expect(await token.symbol()).to.equal("DEFERRED");
    expect(await token.marketplace()).to.equal(marketplace.address);
  });

  it("Should create contract with one seller", async () => {
    const { token, minter, marketplace, rewardPoolContract, alice, bob } =
      deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    // alice should have 40_000 tokens
    expect(await token.balanceOf(alice.address)).to.equal(40_000);
    // get token uri for token id 0
    expect(await token.tokenURI(0)).to.equal("metadataUri");
    // owner of token id 0 should be alice
    expect(await token.ownerOf(0)).to.equal(alice.address);
    // price should be 100
    expect(await token.tokenPriceUsd(0)).to.equal(100);
    // tokens should be approved to marketplace
    expect(await token.getApproved(0)).to.equal(marketplace.address);

    // should have reserved reward
    const expectedReward = 1_000 * 40_000;
    expect(await rewardPoolContract.reservedAmount()).to.equal(expectedReward);
  });

  it("Should not reserve reward if reward is zero", async () => {
    const { token, minter, marketplace, rewardPoolContract, alice, bob } =
      deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address],
      ekokeReward: 0,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    // alice should have 40_000 tokens
    expect(await token.balanceOf(alice.address)).to.equal(40_000);
    // get token uri for token id 0
    expect(await token.tokenURI(0)).to.equal("metadataUri");
    // owner of token id 0 should be alice
    expect(await token.ownerOf(0)).to.equal(alice.address);
    // price should be 100
    expect(await token.tokenPriceUsd(0)).to.equal(100);
    // tokens should be approved to marketplace
    expect(await token.getApproved(0)).to.equal(marketplace.address);

    // should have reserved reward
    const expectedReward = 0;
    expect(await rewardPoolContract.reservedAmount()).to.equal(expectedReward);
  });

  it("Should create a contract with different quotas", async () => {
    const { token, minter, alice, bob, charlie } = deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 60,
        },
        {
          seller: bob.address,
          quota: 40,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [charlie.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    // alice should have 24_000 tokens
    expect(await token.balanceOf(alice.address)).to.equal(24_000);
    // bob should have 16_000 tokens
    expect(await token.balanceOf(bob.address)).to.equal(16_000);
  });

  it("Should create a contract with multiple buyers", async () => {
    const { token, minter, alice, bob, charlie } = deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address, charlie.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });
  });

  it("Should not create a contract with more than 100% quota", async () => {
    const { token, minter, alice, bob } = deploy;

    await expect(
      token.connect(minter).createContract({
        sellers: [
          {
            seller: alice.address,
            quota: 60,
          },
          {
            seller: bob.address,
            quota: 60,
          },
        ],
        metadataUri: "metadataUri",
        buyers: [alice.address],
        ekokeReward: 1_000,
        tokenPriceUsd: 100,
        tokensAmount: 40_000,
      })
    ).to.be.revertedWith("Deferred: total quota must be 100");
  });

  it("Should not create a contract if not minter", async () => {
    const { token, owner, alice } = deploy;

    await expect(
      token.connect(owner).createContract({
        sellers: [
          {
            seller: alice.address,
            quota: 100,
          },
        ],
        metadataUri: "metadataUri",
        buyers: [alice.address],
        ekokeReward: 1_000,
        tokenPriceUsd: 100,
        tokensAmount: 40_000,
      })
    ).to.be.revertedWith("Deferred: caller is not the minter");
  });

  it("Should not create a contract with less than 100% quota", async () => {
    const { token, minter, alice, bob } = deploy;

    await expect(
      token.connect(minter).createContract({
        sellers: [
          {
            seller: alice.address,
            quota: 20,
          },
          {
            seller: bob.address,
            quota: 21,
          },
        ],
        metadataUri: "metadataUri",
        buyers: [alice.address],
        ekokeReward: 1_000,
        tokenPriceUsd: 100,
        tokensAmount: 40_000,
      })
    ).to.be.revertedWith("Deferred: total quota must be 100");
  });

  it("Should not create a contract with a token amount not divisible by 100", async () => {
    const { token, minter, alice } = deploy;

    await expect(
      token.connect(minter).createContract({
        sellers: [
          {
            seller: alice.address,
            quota: 100,
          },
        ],
        metadataUri: "metadataUri",
        buyers: [alice.address],
        ekokeReward: 1_000,
        tokenPriceUsd: 100,
        tokensAmount: 17_123,
      })
    ).to.be.revertedWith("Deferred: tokensAmount must be divisible by 100");
  });

  it("Should not create a contract with 0 tokens", async () => {
    const { token, minter, alice } = deploy;

    await expect(
      token.connect(minter).createContract({
        sellers: [
          {
            seller: alice.address,
            quota: 100,
          },
        ],
        metadataUri: "metadataUri",
        buyers: [alice.address],
        ekokeReward: 1_000,
        tokenPriceUsd: 100,
        tokensAmount: 0,
      })
    ).to.be.revertedWith("Deferred: tokensAmount must be greater than 0");
  });

  it("Should allow marketplace to transfer tokens", async () => {
    const { token, minter, alice, bob, marketplace } = deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    await token
      .connect(marketplace)
      .transferFrom(alice.address, bob.address, 0);

    expect(await token.balanceOf(alice.address)).to.equal(39_999);
    expect(await token.balanceOf(bob.address)).to.equal(1);
    expect(await token.ownerOf(0)).to.equal(bob.address);
  });

  it("Should allow further transfers from marketplace", async () => {
    const { token, minter, alice, bob, charlie, marketplace } = deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    await token
      .connect(marketplace)
      .transferFrom(alice.address, charlie.address, 0);

    expect(await token.balanceOf(alice.address)).to.equal(39_999);
    expect(await token.balanceOf(charlie.address)).to.equal(1);
    expect(await token.ownerOf(0)).to.equal(charlie.address);

    await token
      .connect(marketplace)
      .transferFrom(charlie.address, bob.address, 0);

    expect(await token.balanceOf(bob.address)).to.equal(1);
    expect(await token.balanceOf(charlie.address)).to.equal(0);
    expect(await token.ownerOf(0)).to.equal(bob.address);
  });

  it("Should allow marketplace to transfer tokens", async () => {
    const { token, minter, alice, bob, marketplace } = deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    await token
      .connect(marketplace)
      .transferFrom(alice.address, bob.address, 0);

    expect(await token.balanceOf(alice.address)).to.equal(39_999);
    expect(await token.balanceOf(bob.address)).to.equal(1);
    expect(await token.ownerOf(0)).to.equal(bob.address);
  });

  it("Should not allow approve", async () => {
    const { token, minter, alice, bob, charlie, marketplace } = deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    await expect(
      token.connect(alice).approve(charlie.address, 0)
    ).to.be.revertedWith("Deferred: approve is not allowed");
  });

  it("Should not allow setApprovalForAll", async () => {
    const { token, minter, alice, bob, charlie, marketplace } = deploy;

    await token.connect(minter).createContract({
      sellers: [
        {
          seller: alice.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [bob.address],
      ekokeReward: 1_000,
      tokenPriceUsd: 100,
      tokensAmount: 40_000,
    });

    await expect(
      token.connect(alice).setApprovalForAll(charlie.address, true)
    ).to.be.revertedWith("Deferred: setApprovalForAll is not allowed");
  });
});
