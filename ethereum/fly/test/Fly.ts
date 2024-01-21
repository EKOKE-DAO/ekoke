import { expect } from "chai";
import { ethers } from "hardhat";
import { Fly } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const TOTAL_SUPPLY = "8880101010000000000"; // 8 milions
const NAME = "Fly";
const SYMBOL = "FLY";
const DECIMALS = 12;
const INITIAL_FEE = 100;
const DUMMY_PRINCIPAL = new Uint8Array([
  64, 123, 39, 130, 111, 49, 3, 65, 143, 8, 40, 152, 37, 163, 102, 10, 226, 6,
  132, 148, 181, 23, 75, 76, 77, 109, 126, 107, 2, 14, 0, 10,
]);

describe("Fly", () => {
  interface Contract {
    token: Fly;
    owner: SignerWithAddress;
    flyCanister: SignerWithAddress;
  }

  let deploy: Contract;

  beforeEach(async () => {
    const [owner, otherAccount] = await ethers.getSigners();
    const signer = await ethers.provider.getSigner(owner.address);

    const Contract = await ethers.getContractFactory("Fly");
    const contract = await Contract.deploy(owner.address, INITIAL_FEE);
    await contract.waitForDeployment();

    const address = await contract.getAddress();
    const token = new ethers.Contract(address, Contract.interface, signer);

    deploy = {
      token: token as unknown as Fly,
      owner,
      flyCanister: otherAccount,
    };
  });

  it("Should has the correct name and symbol ", async () => {
    const { token, owner } = deploy;
    expect(await token.name()).to.equal(NAME);
    expect(await token.symbol()).to.equal(SYMBOL);
    expect(await token.decimals()).to.equal(DECIMALS);
    expect(await token.swapFee()).to.equal(INITIAL_FEE);
    // check balance
    expect(await token.balanceOf(owner.address)).to.equal(0);
    // check fly canister is unset
    expect(token.getFlyCanisterAddress()).to.be.revertedWith(
      "Fly: fly canister address not set"
    );
  });

  it("Should set fly canister address just once", async () => {
    const { token, flyCanister } = deploy;
    await token.setFlyCanisterAddress(flyCanister.address);
    expect(await token.getFlyCanisterAddress()).to.equal(flyCanister.address);
    expect(token.setFlyCanisterAddress(flyCanister.address)).to.be.revertedWith(
      "Fly: fly canister address already set"
    );
  });

  it("Should transcribe swap", async () => {
    const { token, owner } = deploy;
    await token.setFlyCanisterAddress(owner.address);
    await token.transcribeSwap(owner.address, 100);
    expect(await token.balanceOf(owner.address)).to.equal(100);
  });

  it("Should swap 100 tokens", async () => {
    const { token, owner, flyCanister } = deploy;
    await token.mintTestnetTokens(owner.address, 100);
    const fee = await token.swapFee();

    await token.setFlyCanisterAddress(flyCanister.address);

    const initialBalance = await ethers.provider.getBalance(owner.address);

    // swap and check event is emitted
    await expect(
      token.swap(DUMMY_PRINCIPAL, 75, {
        value: fee,
      })
    )
      .to.emit(token, "FlySwapped")
      .withArgs(owner.address, DUMMY_PRINCIPAL, 75);

    expect(await token.balanceOf(owner.address)).to.equal(25);

    // check owner has paid FEE ethers
    const finalBalance =
      initialBalance - (await ethers.provider.getBalance(owner.address));

    expect(finalBalance).to.greaterThan(fee);
  });

  it("should fail swap if fly canister address is not set", async () => {
    const { token, owner } = deploy;
    await token.mintTestnetTokens(owner.address, 100);

    const fee = await token.swapFee();

    expect(
      token.swap(DUMMY_PRINCIPAL, 75, {
        value: fee,
      })
    ).to.be.revertedWith("Fly: fly canister address not set");
  });

  it("should fail swap if fee is not paid", async () => {
    const { token, owner, flyCanister } = deploy;
    await token.setFlyCanisterAddress(flyCanister.address);
    await token.mintTestnetTokens(owner.address, 100);

    await expect(
      token.swap(DUMMY_PRINCIPAL, 75, {
        value: 10,
      })
    ).to.be.revertedWith(
      "Fly: caller does not have enough ether to pay the fee"
    );
  });

  it("should fail swap if has not enough tokens", async () => {
    const { token, owner, flyCanister } = deploy;
    await token.setFlyCanisterAddress(flyCanister.address);
    await token.mintTestnetTokens(owner.address, 100);
    const fee = await token.swapFee();

    await expect(
      token.swap(DUMMY_PRINCIPAL, 101, {
        value: fee,
      })
    ).to.be.revertedWith("Fly: caller does not have enough tokens to swap");
  });

  it("Should transfer 500 tokens", async () => {
    const { flyCanister, owner, token } = deploy;
    await token.mintTestnetTokens(owner.address, 1_000);

    await token.transfer(flyCanister.address, 250);
    expect(await token.balanceOf(flyCanister.address)).to.equal(250);
    expect(await token.balanceOf(owner.address)).to.equal(750);
  });

  it("should get total supply and swapped supply", async () => {
    const { owner, token } = deploy;
    await token.mintTestnetTokens(owner.address, 1_000);

    expect(await token.totalSupply()).to.equal(TOTAL_SUPPLY);
    expect(await token.swappedSupply()).to.equal(1_000);
  });

  it("Should update swap fee", async () => {
    const { token } = deploy;
    await token.setSwapFee(200);
    expect(await token.swapFee()).to.equal(200);
  });

  it("should renounce ownership", async () => {
    const { token } = deploy;
    await token.renounceOwnership();
    expect(await token.owner()).to.equal(
      "0x0000000000000000000000000000000000000000"
    );
  });

  it("should mint testnet tokens", async () => {
    const { flyCanister, token } = deploy;
    await token.mintTestnetTokens(flyCanister.address, 1_000);
    expect(await token.balanceOf(flyCanister.address)).to.equal(1_000);
  });

  it("should transfer ownership of the contract", async () => {
    const { flyCanister, owner: originalOwner, token } = deploy;
    await token.transferOwnership(flyCanister.address);
    expect(await token.owner()).to.equal(flyCanister.address);
  });
});
