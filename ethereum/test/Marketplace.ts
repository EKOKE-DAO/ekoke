import { expect } from "chai";
import { ethers } from "hardhat";
import { RewardPool, Ekoke, Deferred, Marketplace } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const EKOKE_REWARD = 1000;
const USD_PRICE = 100;

describe("RewardPool", () => {
  interface Contract {
    marketplace: Marketplace;
    deferred: Deferred;
    rewardPool: RewardPool;
    ekoke: Ekoke;
    usdt: Ekoke;
    owner: SignerWithAddress;
    seller: SignerWithAddress;
    buyer: SignerWithAddress;
    thirdParty: SignerWithAddress;
    minter: SignerWithAddress;
  }

  let deploy: Contract;

  beforeEach(async () => {
    const [owner, buyer, seller, minter, thirdParty] =
      await ethers.getSigners();

    const ekokeContract = await ethers.deployContract("Ekoke", [owner.address]);
    const ekoke = ekokeContract as unknown as Ekoke;

    const usdtContract = await ethers.deployContract("Ekoke", [owner.address]);
    const usdt = usdtContract as unknown as Ekoke;
    // mint 1000 USDT to alice
    await usdt.adminMint(buyer.address, 1000);
    await usdt.adminMint(thirdParty.address, 1000);

    const deferredContract = await ethers.deployContract("Deferred", [
      owner.address,
    ]);
    const deferred = deferredContract as unknown as Deferred;

    const rewardPoolContract = await ethers.deployContract("RewardPool", [
      owner.address,
      ekoke.getAddress(),
      deferred.getAddress(),
    ]);
    const rewardPool = rewardPoolContract as unknown as RewardPool;

    const marketplaceContract = await ethers.deployContract("Marketplace", [
      owner.address,
      usdt.getAddress(),
      ekoke.getAddress(),
      deferred.getAddress(),
    ]);
    const marketplace = marketplaceContract as unknown as Marketplace;

    // set contracts
    await rewardPool.adminSetMarketplace(marketplace.getAddress());
    await ekoke.adminSetRewardPoolAddress(rewardPool.getAddress());
    await deferred.adminSetDeferredMinter(minter.address);
    await deferred.adminSetMarketplace(marketplace.getAddress());
    await deferred.adminSetRewardPool(rewardPool.getAddress());
    await marketplace.adminSetRewardPool(rewardPool.getAddress());

    // create a sell contract
    await deferred.connect(minter).createContract({
      contractId: 1,
      sellers: [
        {
          seller: seller.address,
          quota: 100,
        },
      ],
      metadataUri: "metadataUri",
      buyers: [buyer.address],
      ekokeReward: EKOKE_REWARD,
      tokenPriceUsd: USD_PRICE,
      tokensAmount: 40_000,
    });

    deploy = {
      rewardPool,
      ekoke,
      owner,
      marketplace,
      deferred,
      buyer,
      seller,
      minter,
      usdt,
      thirdParty,
    };
  });

  it("Should buy a NFT with USDT as third-party", async () => {
    const { marketplace, thirdParty, seller, deferred, ekoke, usdt } = deploy;

    const tokenId = 0;
    // give allowance to marketplace
    await usdt.connect(thirdParty).approve(marketplace.getAddress(), USD_PRICE);
    // buy
    await marketplace.connect(thirdParty).buyToken(tokenId);

    // USDT balance of buyer
    expect(await usdt.balanceOf(thirdParty.address)).to.equal(1000 - USD_PRICE);
    // USDT balance of seller
    expect(await usdt.balanceOf(seller.address)).to.equal(USD_PRICE);

    // check NFT has been transferred
    expect(await deferred.ownerOf(tokenId)).to.equal(thirdParty.address);

    // check buyer has received the reward
    expect(await ekoke.balanceOf(thirdParty.address)).to.equal(EKOKE_REWARD);
  });

  it("Should buy a NFT with USDT as contract buyer", async () => {
    const { marketplace, buyer, seller, deferred, ekoke, usdt } = deploy;

    const tokenId = 0;
    const interest = 10;
    // give allowance to marketplace
    await usdt
      .connect(buyer)
      .approve(marketplace.getAddress(), USD_PRICE + interest);
    // buy
    await marketplace.connect(buyer).buyToken(tokenId);

    // USDT balance of buyer
    expect(await usdt.balanceOf(buyer.address)).to.equal(
      1000 - USD_PRICE - interest
    );
    // USDT balance of seller
    expect(await usdt.balanceOf(seller.address)).to.equal(USD_PRICE);
    // USDT balance of marketplace
    expect(await usdt.balanceOf(marketplace.getAddress())).to.equal(interest);

    // check NFT has been transferred
    expect(await deferred.ownerOf(tokenId)).to.equal(buyer.address);

    // check buyer has received the reward
    expect(await ekoke.balanceOf(buyer.address)).to.equal(EKOKE_REWARD);
  });

  it("Should get token price as contract buyer", async () => {
    const { buyer, marketplace } = deploy;

    const tokenId = 0;
    const interest = 10;
    // give allowance to marketplace
    expect(
      await marketplace.connect(buyer).tokenPriceForCaller(tokenId)
    ).to.equal(USD_PRICE + (USD_PRICE * interest) / 100);
  });

  it("Should get token price as third-party", async () => {
    const { marketplace, thirdParty } = deploy;

    const tokenId = 0;
    // give allowance to marketplace
    expect(
      await marketplace.connect(thirdParty).tokenPriceForCaller(tokenId)
    ).to.equal(USD_PRICE);
  });

  it("Should set interest rate", async () => {
    const { marketplace } = deploy;

    await marketplace.adminSetInterestRate(15);
    expect(await marketplace.interestRate()).to.equal(15);
  });

  it("Should set interest rate to 100", async () => {
    const { marketplace } = deploy;

    await marketplace.adminSetInterestRate(100);
    expect(await marketplace.interestRate()).to.equal(100);
  });

  it("Should not set interest rate if not owner", async () => {
    const { marketplace, thirdParty } = deploy;

    await expect(
      marketplace.connect(thirdParty).adminSetInterestRate(15)
    ).to.be.rejectedWith(Error);
  });

  it("Should not allow interest rate of zero", async () => {
    const { marketplace } = deploy;

    await expect(marketplace.adminSetInterestRate(0)).to.be.revertedWith(
      "Marketplace: Interest rate must be greater than 0"
    );
  });

  it("Should not allow interest rate higher than 100", async () => {
    const { marketplace } = deploy;

    await expect(marketplace.adminSetInterestRate(101)).to.be.revertedWith(
      "Marketplace: Interest rate must be less than 100"
    );
  });
});
