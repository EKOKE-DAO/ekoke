import { HardhatUserConfig, task } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

require("dotenv").config();

const {
  ETHEREUM_API_URL,
  DEV_PRIVATE_KEY,
  PROD_PRIVATE_KEY,
  LOCAL_PRIVATE_KEY,
  ETHERSCAN_API_KEY,
  SEPOLIA_API_URL,
} = process.env;

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.28",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  sourcify: {
    enabled: true,
  },
  etherscan: {
    apiKey: ETHERSCAN_API_KEY,
  },
  networks: {
    ethereum: {
      url: ETHEREUM_API_URL,
      accounts: [`0x${PROD_PRIVATE_KEY}`],
    },
    sepolia: {
      url: SEPOLIA_API_URL,
      accounts: [`0x${DEV_PRIVATE_KEY}`],
    },
    localhost: {
      url: "http://127.0.0.1:8545/",
      accounts: [`0x${LOCAL_PRIVATE_KEY}`],
    },
  },
  gasReporter: {
    currency: "USD",
    enabled: true,
    gasPriceApi:
      "https://api.etherscan.io/api?module=proxy&action=eth_gasPrice",
  },
};

export default config;

const { OWNER_ADDRESS } = process.env;

interface MarketplaceArgs {
  deferred: string;
  ekoke: string;
  usdErc20: string;
}

interface RewardPoolArgs {
  deferred: string;
  ekoke: string;
}

const CONTRACT_DEFERRED = "deferred";
const CONTRACT_EKOKE = "ekoke";
const CONTRACT_MARKETPLACE = "marketplace";
const CONTRACT_REWARD_POOL = "reward-pool";

task("deploy", "Deploy contracts")
  .addPositionalParam(
    "contract",
    "Contract to deploy (marketplace, deferred, ekoke, reward-pool)"
  )
  .addOptionalParam("deferred", "Deferred contract address")
  .addOptionalParam("ekoke", "Ekoke contract address")
  .addOptionalParam("usderc20", "USD ERC20 contract address")
  .setAction(async (taskArgs, hre) => {
    const contract = taskArgs.contract;
    console.log(`Deploying contract ${contract}`);

    switch (contract) {
      case CONTRACT_DEFERRED:
        await deployDeferred();
        break;

      case CONTRACT_EKOKE:
        await deployEkoke();
        break;

      case CONTRACT_MARKETPLACE:
        await deployMarketplace({
          deferred: taskArgs.deferred,
          ekoke: taskArgs.ekoke,
          usdErc20: taskArgs.usderc20,
        });
        break;

      case CONTRACT_REWARD_POOL:
        await deployRewardPool({
          deferred: taskArgs.deferred,
          ekoke: taskArgs.ekoke,
        });
        break;

      default:
        console.error(`Unknown contract: ${contract}`);
        process.exit(1);
    }
  });

async function deployDeferred() {
  const hardhat = require("hardhat");
  const Contract = await hardhat.ethers.getContractFactory("Deferred");
  const contract = await Contract.deploy(OWNER_ADDRESS!);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`Deferred deployed to ${address}`);
}

async function deployEkoke() {
  const hardhat = require("hardhat");
  const Contract = await hardhat.ethers.getContractFactory("Ekoke");
  const contract = await Contract.deploy(OWNER_ADDRESS!);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`EKOKE deployed to ${address}`);
}

async function deployMarketplace(args: MarketplaceArgs) {
  const hardhat = require("hardhat");
  const Contract = await hardhat.ethers.getContractFactory("Marketplace");
  const contract = await Contract.deploy(
    OWNER_ADDRESS!,
    args.usdErc20,
    args.ekoke,
    args.deferred
  );
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`Marketplace deployed to ${address}`);
}

async function deployRewardPool(args: RewardPoolArgs) {
  const hardhat = require("hardhat");
  const Contract = await hardhat.ethers.getContractFactory("RewardPool");
  const contract = await Contract.deploy(
    OWNER_ADDRESS!,
    args.ekoke,
    args.deferred
  );
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`Reward pool deployed to ${address}`);
}
