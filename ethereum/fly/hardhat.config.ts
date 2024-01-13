import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import { task } from "hardhat/config";
require("dotenv").config();

const {
  ETHEREUM_API_URL,
  GOERLI_API_URL,
  DEV_PRIVATE_KEY,
  PROD_PRIVATE_KEY,
  LOCAL_PRIVATE_KEY,
} = process.env;

const config: HardhatUserConfig = {
  solidity: "0.8.20",
  networks: {
    ethereum: {
      url: ETHEREUM_API_URL,
      accounts: [`0x${PROD_PRIVATE_KEY}`],
    },
    goerli: {
      url: GOERLI_API_URL,
      accounts: [`0x${DEV_PRIVATE_KEY}`],
    },
    localhost: {
      url: "http://127.0.0.1:8545/",
      accounts: [`0x${LOCAL_PRIVATE_KEY}`],
    },
  },
  gasReporter: {
    currency: "USD",
    gasPriceApi:
      "https://api.etherscan.io/api?module=proxy&action=eth_gasPrice",
  },
};

export default config;
