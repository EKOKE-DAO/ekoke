import { expect } from "chai";
import { ethers } from "hardhat";
import {
  RewardPool,
  Ekoke,
  Deferred,
  Marketplace,
  TestERC20,
} from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const EKOKE_REWARD = 1000;
const USD_PRICE = 100;
const USDT_DECIMALS = 6;
const CONTRACT_ID = 1;

const usdToUsdt = (usd: number) => usd * 10 ** USDT_DECIMALS;

const INITIAL_USDT_BALANCE = usdToUsdt(1000);

describe("RewardPool", () => {
  interface Contract {
    marketplace: Marketplace;
    deferred: Deferred;
    rewardPool: RewardPool;
    ekoke: Ekoke;
    usdt: TestERC20;
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

    const usdtContract = await ethers.deployContract("TestERC20", [
      "USDT",
      "USDT",
      USDT_DECIMALS,
    ]);
    const usdt = usdtContract as unknown as TestERC20;
    // mint 1000 USDT to alice
    await usdt.mint(buyer.address, INITIAL_USDT_BALANCE);
    await usdt.mint(thirdParty.address, INITIAL_USDT_BALANCE);

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
      contractId: CONTRACT_ID,
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

    // give allowance to marketplace
    await usdt
      .connect(thirdParty)
      .approve(marketplace.getAddress(), usdToUsdt(USD_PRICE));
    // buy
    const expectedTokenId = await deferred.nextTokenIdToBuyFor(
      CONTRACT_ID,
      thirdParty.address
    );
    await marketplace.connect(thirdParty).buyNextToken(CONTRACT_ID);
    expect(await deferred.ownerOf(expectedTokenId)).to.equal(
      thirdParty.address
    );

    // USDT balance of buyer
    expect(await usdt.balanceOf(thirdParty.address)).to.equal(
      INITIAL_USDT_BALANCE - usdToUsdt(USD_PRICE)
    );
    // USDT balance of seller
    expect(await usdt.balanceOf(seller.address)).to.equal(usdToUsdt(USD_PRICE));

    // check NFT has been transferred
    expect(await deferred.ownerOf(expectedTokenId)).to.equal(
      thirdParty.address
    );

    // check buyer has received the reward
    expect(await ekoke.balanceOf(thirdParty.address)).to.equal(EKOKE_REWARD);
  });

  it("Should buy a NFT with USDT as contract buyer", async () => {
    const { marketplace, buyer, usdt, seller, deferred, ekoke } = deploy;

    const interest = 10;
    // give allowance to marketplace
    await usdt
      .connect(buyer)
      .approve(
        marketplace.getAddress(),
        usdToUsdt(USD_PRICE) + usdToUsdt(interest)
      );
    // buy
    const expectedTokenId = await deferred.nextTokenIdToBuyFor(
      CONTRACT_ID,
      buyer.address
    );
    await marketplace.connect(buyer).buyNextToken(CONTRACT_ID);
    expect(await deferred.ownerOf(expectedTokenId)).to.equal(buyer.address);

    // USDT balance of buyer
    expect(await usdt.balanceOf(buyer.address)).to.equal(
      INITIAL_USDT_BALANCE - usdToUsdt(USD_PRICE) - usdToUsdt(interest)
    );
    // USDT balance of seller
    expect(await usdt.balanceOf(seller.address)).to.equal(usdToUsdt(USD_PRICE));
    // USDT balance of marketplace
    expect(await usdt.balanceOf(marketplace.getAddress())).to.equal(
      usdToUsdt(interest)
    );

    // check NFT has been transferred
    expect(await deferred.ownerOf(expectedTokenId)).to.equal(buyer.address);

    // check buyer has received the reward
    expect(await ekoke.balanceOf(buyer.address)).to.equal(EKOKE_REWARD);
  });

  it("Should get token price as contract buyer", async () => {
    const { buyer, marketplace } = deploy;

    const interest = 10;
    // give allowance to marketplace
    expect(
      await marketplace.connect(buyer).tokenPriceForCaller(CONTRACT_ID)
    ).to.equal(usdToUsdt(USD_PRICE) + (usdToUsdt(USD_PRICE) * interest) / 100);
  });

  it("Should get token price as third-party", async () => {
    const { marketplace, thirdParty } = deploy;

    // give allowance to marketplace
    expect(
      await marketplace.connect(thirdParty).tokenPriceForCaller(CONTRACT_ID)
    ).to.equal(usdToUsdt(USD_PRICE));
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
