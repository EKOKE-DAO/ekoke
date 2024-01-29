import { expect } from "chai";
import { ethers } from "hardhat";
import { Ekoke } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const TOTAL_SUPPLY = "8880101010000000000"; // 8 milions
const NAME = "Ekoke";
const SYMBOL = "EKOKE";
const DECIMALS = 12;
const DUMMY_PRINCIPAL = new Uint8Array([
  64, 123, 39, 130, 111, 49, 3, 65, 143, 8, 40, 152, 37, 163, 102, 10, 226, 6,
  132, 148, 181, 23, 75, 76, 77, 109, 126, 107, 2, 14, 0, 10,
]);

describe("Ekoke", () => {
  interface Contract {
    token: Ekoke;
    owner: SignerWithAddress;
    ekokeCanister: SignerWithAddress;
  }

  let deploy: Contract;

  beforeEach(async () => {
    const [owner, otherAccount] = await ethers.getSigners();
    const signer = await ethers.provider.getSigner(owner.address);

    const Contract = await ethers.getContractFactory("Ekoke");
    const contract = await Contract.deploy(owner.address);
    await contract.waitForDeployment();

    const address = await contract.getAddress();
    const token = new ethers.Contract(address, Contract.interface, signer);

    deploy = {
      token: token as unknown as Ekoke,
      owner,
      ekokeCanister: otherAccount,
    };
  });

  it("Should has the correct name and symbol ", async () => {
    const { token, owner } = deploy;
    expect(await token.name()).to.equal(NAME);
    expect(await token.symbol()).to.equal(SYMBOL);
    expect(await token.decimals()).to.equal(DECIMALS);
    // check balance
    expect(await token.balanceOf(owner.address)).to.equal(0);
    // check ekoke canister is unset
    expect(token.getEkokeCanisterAddress()).to.be.revertedWith(
      "Ekoke: ekoke canister address not set"
    );
  });

  it("Should set ekoke canister address just once", async () => {
    const { token, ekokeCanister } = deploy;
    await token.setEkokeCanisterAddress(ekokeCanister.address);
    expect(await token.getEkokeCanisterAddress()).to.equal(
      ekokeCanister.address
    );
    expect(
      token.setEkokeCanisterAddress(ekokeCanister.address)
    ).to.be.revertedWith("Ekoke: ekoke canister address already set");
  });

  it("Should transcribe swap", async () => {
    const { token, owner } = deploy;
    await token.setEkokeCanisterAddress(owner.address);
    await token.transcribeSwap(owner.address, 100);
    expect(await token.balanceOf(owner.address)).to.equal(100);
  });

  it("Should swap 100 tokens", async () => {
    const { token, owner, ekokeCanister } = deploy;
    await token.mintTestnetTokens(owner.address, 100);

    await token.setEkokeCanisterAddress(ekokeCanister.address);
    const initialEkokeCanisterBalance = await ethers.provider.getBalance(
      ekokeCanister.address
    );

    const initialBalance = await ethers.provider.getBalance(owner.address);

    // swap and check event is emitted
    await expect(token.swap(DUMMY_PRINCIPAL, 75))
      .to.emit(token, "EkokeSwapped")
      .withArgs(owner.address, DUMMY_PRINCIPAL, 75);

    expect(await token.balanceOf(owner.address)).to.equal(25);
  });

  it("should fail swap if ekoke canister address is not set", async () => {
    const { token, owner } = deploy;
    await token.mintTestnetTokens(owner.address, 100);

    expect(token.swap(DUMMY_PRINCIPAL, 75)).to.be.revertedWith(
      "Ekoke: ekoke canister address not set"
    );
  });

  it("should fail swap if has not enough tokens", async () => {
    const { token, owner, ekokeCanister } = deploy;
    await token.setEkokeCanisterAddress(ekokeCanister.address);
    await token.mintTestnetTokens(owner.address, 100);

    await expect(token.swap(DUMMY_PRINCIPAL, 101)).to.be.revertedWith(
      "Ekoke: caller does not have enough tokens to swap"
    );
  });

  it("Should transfer 500 tokens", async () => {
    const { ekokeCanister, owner, token } = deploy;
    await token.mintTestnetTokens(owner.address, 1_000);

    await token.transfer(ekokeCanister.address, 250);
    expect(await token.balanceOf(ekokeCanister.address)).to.equal(250);
    expect(await token.balanceOf(owner.address)).to.equal(750);
  });

  it("should get total supply and swapped supply", async () => {
    const { owner, token } = deploy;
    await token.mintTestnetTokens(owner.address, 1_000);

    expect(await token.totalSupply()).to.equal(TOTAL_SUPPLY);
    expect(await token.swappedSupply()).to.equal(1_000);
  });

  it("should renounce ownership", async () => {
    const { token } = deploy;
    await token.renounceOwnership();
    expect(await token.owner()).to.equal(
      "0x0000000000000000000000000000000000000000"
    );
  });

  it("should mint testnet tokens", async () => {
    const { ekokeCanister, token } = deploy;
    await token.mintTestnetTokens(ekokeCanister.address, 1_000);
    expect(await token.balanceOf(ekokeCanister.address)).to.equal(1_000);
  });

  it("should transfer ownership of the contract", async () => {
    const { ekokeCanister, owner: originalOwner, token } = deploy;
    await token.transferOwnership(ekokeCanister.address);
    expect(await token.owner()).to.equal(ekokeCanister.address);
  });
});
