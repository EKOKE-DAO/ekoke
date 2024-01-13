import { expect } from "chai";
import { ethers } from "hardhat";
import { MyToken } from "../typechain-types";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const INITIAL_SUPPLY = "8880101010000000000"; // 8 milions
const DECIMALS = 12;

describe("Fly", () => {
  interface Contract {
    token: MyToken;
    owner: SignerWithAddress;
    otherAccount: SignerWithAddress;
  }

  let deploy: Contract;
  let _name = "Fly";
  let _symbol = "FLY";

  beforeEach(async () => {
    const [owner, otherAccount] = await ethers.getSigners();
    const contract = await ethers.deployContract(_name, [
      _name,
      _symbol,
      owner.address,
      INITIAL_SUPPLY,
      DECIMALS,
    ]);

    deploy = {
      token: contract as unknown as MyToken,
      owner,
      otherAccount,
    };
  });

  it("Should has the correct name and symbol ", async () => {
    const { token, owner } = deploy;
    expect(await token.name()).to.equal(_name);
    expect(await token.symbol()).to.equal(_symbol);
    expect(await token.decimals()).to.equal(DECIMALS);
    // check balance
    expect(await token.balanceOf(owner.address)).to.equal(INITIAL_SUPPLY);
  });

  it("Should transfer 500 tokens", async () => {
    const { otherAccount, token } = deploy;
    await token.transfer(otherAccount.address, 500);
    expect(await token.balanceOf(otherAccount.address)).to.equal(500);
  });

  it("should renounce ownership", async () => {
    const { owner, token } = deploy;
    await token.renounceOwnership();
    expect(await token.owner()).to.equal(
      "0x0000000000000000000000000000000000000000"
    );
  });

  it("should transfer ownership of the NFT", async () => {
    const { otherAccount, owner: originalOwner, token } = deploy;
    await token.transferOwnership(otherAccount.address);
    expect(await token.owner()).to.equal(otherAccount.address);
  });
});
