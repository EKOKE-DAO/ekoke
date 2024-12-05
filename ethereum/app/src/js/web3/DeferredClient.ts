import Web3 from 'web3';

import { ABI, CONTRACT_ADDRESS } from './contracts/Deferred';
import { ChainId } from '../components/MetamaskConnect';

interface CreateContractArgs {
  contractId: bigint;
  sellers: { seller: string; quota: number }[];
  metadataUri: string;
  buyers: string[];
  ekokeReward: bigint;
  tokenPriceUsd: bigint;
  tokensAmount: bigint;
}

export default class DeferredClient {
  private address: string;
  private web3: Web3;
  private chainId: ChainId;

  constructor(address: string, ethereum: any, chainId: ChainId) {
    this.address = address;
    this.web3 = new Web3(ethereum);
    this.chainId = chainId;
  }

  async transferOwnership(newAddress: string) {
    const contract = this.getContract();
    return contract.methods
      .transferOwnership(newAddress)
      .send({ from: this.address });
  }

  async adminSetDeferredMinter(newAddress: string) {
    const contract = this.getContract();
    return contract.methods
      .adminSetDeferredMinter(newAddress)
      .send({ from: this.address });
  }

  async adminSetMarketplace(newAddress: string) {
    const contract = this.getContract();
    return contract.methods
      .adminSetMarketplace(newAddress)
      .send({ from: this.address });
  }

  async adminSetRewardPool(newAddress: string) {
    const contract = this.getContract();
    return contract.methods
      .adminSetRewardPool(newAddress)
      .send({ from: this.address });
  }

  async createContract(args: CreateContractArgs) {
    const contract = this.getContract();
    return contract.methods.createContract(args).send({ from: this.address });
  }

  async deferredMinter(): Promise<string> {
    const contract = this.getContract();
    return contract.methods.deferredMinter().call();
  }

  async marketplace(): Promise<string> {
    const contract = this.getContract();
    return contract.methods.marketplace().call();
  }

  async rewardPool(): Promise<string> {
    const contract = this.getContract();
    return contract.methods.rewardPool().call();
  }

  async ownerOf(tokenId: number): Promise<string> {
    const contract = this.getContract();
    return contract.methods.ownerOf(tokenId).call();
  }

  private getContract() {
    return new this.web3.eth.Contract(ABI, CONTRACT_ADDRESS[this.chainId]);
  }
}
