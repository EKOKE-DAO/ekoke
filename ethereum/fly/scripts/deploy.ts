import { ethers } from "hardhat";
require("dotenv").config();

const { OWNER_ADDRESS } = process.env;

async function main() {
  // deploy contract
  const Contract = await ethers.getContractFactory("MyToken");
  const contract = await Contract.deploy(
    "Fly",
    "FLY",
    OWNER_ADDRESS!,
    "8880101010000000000",
    12
  );
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`Contract deployed to ${address}`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
