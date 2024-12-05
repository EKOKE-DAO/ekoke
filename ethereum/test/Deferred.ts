import { expect } from "chai";
import { ethers } from "hardhat";
import { Deferred, RewardPool, Ekoke } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const NAME = "Deferred";

describe("Deferred", () => {
  interface Contract {
    deferred: Deferred;
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
    const deferred = contract as unknown as Deferred;

    const ekokeContract = await ethers.deployContract("Ekoke", [owner.address]);
    const ekoke = ekokeContract as unknown as Ekoke;

    const rewardPoolContract = await ethers.deployContract("RewardPool", [
      owner.address,
      ekoke.getAddress(),
      contract.getAddress(),
    ]);
    const rewardPool = rewardPoolContract as unknown as RewardPool;

    // set contracts
    await deferred.adminSetDeferredMinter(minter.address);
    await deferred.adminSetRewardPool(rewardPool.getAddress());
    await deferred.adminSetMarketplace(marketplace.address);

    deploy = {
      deferred,
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
    const { deferred, marketplace } = deploy;
    expect(await deferred.name()).to.equal(NAME);
    expect(await deferred.symbol()).to.equal("DEFERRED");
    expect(await deferred.marketplace()).to.equal(marketplace.address);
  });

  it("Should create contract with one seller", async () => {
    const { deferred, minter, marketplace, rewardPoolContract, alice, bob } =
      deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
    expect(await deferred.balanceOf(alice.address)).to.equal(40_000);
    // get token uri for token id 0
    expect(await deferred.tokenURI(0)).to.equal("metadataUri");
    // owner of token id 0 should be alice
    expect(await deferred.ownerOf(0)).to.equal(alice.address);
    // price should be 100
    expect(await deferred.tokenPriceUsd(0)).to.equal(100);
    // tokens should be approved to marketplace
    expect(await deferred.getApproved(0)).to.equal(marketplace.address);

    // should have reserved reward
    const expectedReward = 1_000 * 40_000;
    expect(await rewardPoolContract.reservedAmount()).to.equal(expectedReward);
  });

  it("Should not reserve reward if reward is zero", async () => {
    const { deferred, minter, marketplace, rewardPoolContract, alice, bob } =
      deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
    expect(await deferred.balanceOf(alice.address)).to.equal(40_000);
    // get token uri for token id 0
    expect(await deferred.tokenURI(0)).to.equal("metadataUri");
    // owner of token id 0 should be alice
    expect(await deferred.ownerOf(0)).to.equal(alice.address);
    // price should be 100
    expect(await deferred.tokenPriceUsd(0)).to.equal(100);
    // tokens should be approved to marketplace
    expect(await deferred.getApproved(0)).to.equal(marketplace.address);

    // should have reserved reward
    const expectedReward = 0;
    expect(await rewardPoolContract.reservedAmount()).to.equal(expectedReward);
  });

  it("Should create a contract with different quotas", async () => {
    const { deferred, minter, alice, bob, charlie } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
    expect(await deferred.balanceOf(alice.address)).to.equal(24_000);
    // bob should have 16_000 tokens
    expect(await deferred.balanceOf(bob.address)).to.equal(16_000);
  });

  it("Should not create a contract with a duplicate id", async () => {
    const { deferred, minter, alice, bob, charlie } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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

    await expect(
      deferred.connect(minter).createContract({
        contractId: 1,
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
      })
    ).to.be.revertedWith("Deferred: contract is already created");
  });

  it("Should create a contract with multiple buyers", async () => {
    const { deferred, minter, alice, bob, charlie } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
    const { deferred, minter, alice, bob } = deploy;

    await expect(
      deferred.connect(minter).createContract({
        contractId: 1,
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
    const { deferred, owner, alice } = deploy;

    await expect(
      deferred.connect(owner).createContract({
        contractId: 1,
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
    const { deferred, minter, alice, bob } = deploy;

    await expect(
      deferred.connect(minter).createContract({
        contractId: 1,
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
    const { deferred, minter, alice } = deploy;

    await expect(
      deferred.connect(minter).createContract({
        contractId: 1,
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
    const { deferred, minter, alice } = deploy;

    await expect(
      deferred.connect(minter).createContract({
        contractId: 1,
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

  it("Should close a contract", async () => {
    const { deferred, minter, alice, bob, charlie } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
    expect(await deferred.balanceOf(alice.address)).to.equal(24_000);
    // bob should have 16_000 tokens
    expect(await deferred.balanceOf(bob.address)).to.equal(16_000);

    const contractId = 1;

    // close the contract
    await deferred.connect(minter).closeContract(contractId);
  });

  it("Should not return closed contracts", async () => {
    const { deferred, minter, alice, bob, charlie } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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

    expect((await deferred.tokenContract(0)).closed).to.equal(false);

    // close the contract
    await deferred.connect(minter).closeContract(1);

    await expect(deferred.tokenContract(1)).to.be.revertedWith(
      "Deferred: token does not exist"
    );
  });

  it("Should allow marketplace to transfer tokens", async () => {
    const { deferred, minter, alice, bob, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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

    const tokenId = await deferred.nextTokenIdToBuyFor(1, bob.address);

    await deferred
      .connect(marketplace)
      .transferToken(1, alice.address, bob.address);

    expect(await deferred.ownerOf(tokenId)).to.equal(bob.address);

    expect(await deferred.balanceOf(alice.address)).to.equal(39_999);
    expect(await deferred.balanceOf(bob.address)).to.equal(1);
    expect(await deferred.ownerOf(0)).to.equal(bob.address);
  });

  it("Should allow further transfers from marketplace", async () => {
    const { deferred, minter, alice, bob, charlie, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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

    const tokenIdForThirdParty = await deferred.nextTokenIdToBuyFor(
      1,
      charlie.address
    );

    await deferred
      .connect(marketplace)
      .transferToken(1, alice.address, charlie.address);

    expect(await deferred.ownerOf(tokenIdForThirdParty)).to.equal(
      charlie.address
    );

    expect(await deferred.balanceOf(alice.address)).to.equal(39_999);
    expect(await deferred.balanceOf(charlie.address)).to.equal(1);
    expect(await deferred.ownerOf(0)).to.equal(charlie.address);

    const tokenIdForBuyer = await deferred.nextTokenIdToBuyFor(1, bob.address);

    await deferred
      .connect(marketplace)
      .transferToken(1, charlie.address, bob.address);
    expect(await deferred.ownerOf(tokenIdForBuyer)).to.equal(bob.address);

    expect(await deferred.balanceOf(bob.address)).to.equal(1);
    expect(await deferred.balanceOf(charlie.address)).to.equal(0);
    expect(await deferred.ownerOf(0)).to.equal(bob.address);
  });

  it("Should allow marketplace to transfer tokens", async () => {
    const { deferred, minter, alice, bob, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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

    const nextTokenId = await deferred.nextTokenIdToBuyFor(1, bob.address);

    await deferred
      .connect(marketplace)
      .transferToken(1, alice.address, bob.address);
    expect(await deferred.ownerOf(nextTokenId)).to.equal(bob.address);

    expect(await deferred.balanceOf(alice.address)).to.equal(39_999);
    expect(await deferred.balanceOf(bob.address)).to.equal(1);
    expect(await deferred.ownerOf(0)).to.equal(bob.address);
  });

  it("Should increment next third party token id, if the buyer has bought first", async () => {
    const { deferred, minter, alice, bob, charlie, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
      tokensAmount: 100,
    });

    const tokenId = await deferred.nextTokenIdToBuyFor(1, bob.address);
    await deferred
      .connect(marketplace)
      .transferToken(1, alice.address, bob.address);

    // check third party next token id has incremented
    const expected = tokenId + BigInt(1);
    expect(await deferred.nextTokenIdToBuyFor(1, charlie.address)).to.equal(
      expected
    );
    expect(await deferred.nextTokenIdToBuyFor(1, bob.address)).to.equal(
      expected
    );

    const expected2 = tokenId + BigInt(2);
    // charlie buys
    await deferred
      .connect(marketplace)
      .transferToken(1, alice.address, charlie.address);
    expect(await deferred.nextTokenIdToBuyFor(1, charlie.address)).to.equal(
      expected2
    );
    // for bob should be the same
    expect(await deferred.nextTokenIdToBuyFor(1, bob.address)).to.equal(
      expected
    );

    expect(await deferred.balanceOf(alice.address)).to.equal(98);
  });

  it("Should fail bad token owner for lazy minting", async () => {
    const { deferred, minter, alice, bob, charlie, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
      tokensAmount: 100,
    });

    await expect(
      deferred
        .connect(marketplace)
        .transferToken(1, charlie.address, bob.address)
    ).to.be.revertedWith("Deferred: from is not the owner of the token");
  });

  it("Should tell whether contract is completed", async () => {
    const { deferred, minter, alice, bob, charlie, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
      tokensAmount: 100,
    });

    expect(await deferred.contractCompleted(1)).to.equal(false);
    expect(await deferred.contractProgress(1)).to.equal(0);

    for (let i = 0; i < 100; i++) {
      const tokenId = await deferred.nextTokenIdToBuyFor(1, bob.address);

      await deferred
        .connect(marketplace)
        .transferToken(1, alice.address, charlie.address);

      expect(await deferred.ownerOf(tokenId)).to.equal(charlie.address);
      expect(await deferred.contractProgress(1)).to.equal(i);

      await deferred
        .connect(marketplace)
        .transferToken(1, charlie.address, bob.address);

      expect(await deferred.ownerOf(tokenId)).to.equal(bob.address);
      expect(await deferred.contractProgress(1)).to.equal(i + 1);
    }

    expect(await deferred.contractCompleted(1)).to.equal(true);
  });

  it("Should not allow approve", async () => {
    const { deferred, minter, alice, bob, charlie, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
      deferred.connect(alice).approve(charlie.address, 0)
    ).to.be.revertedWith("Deferred: approve is not allowed");
  });

  it("Should not allow setApprovalForAll", async () => {
    const { deferred, minter, alice, bob, charlie } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
      deferred.connect(alice).setApprovalForAll(charlie.address, true)
    ).to.be.revertedWith("Deferred: setApprovalForAll is not allowed");
  });

  it("should not allow transferFrom", async () => {
    const { deferred, minter, alice, bob, charlie, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
      deferred.connect(alice).transferFrom(alice.address, charlie.address, 0)
    ).to.be.revertedWith("Deferred: transferFrom is not allowed");
  });

  it("Should not allow safeTransferFrom", async () => {
    const { deferred, minter, alice, bob, charlie, marketplace } = deploy;

    await deferred.connect(minter).createContract({
      contractId: 1,
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
      deferred
        .connect(alice)
        .safeTransferFrom(alice.address, charlie.address, 0)
    ).to.be.revertedWith("Deferred: safeTransferFrom is not allowed");
  });
});
