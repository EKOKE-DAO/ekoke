import Web3 from 'web3';

import { ABI, CONTRACT_ADDRESS } from './contracts/RewardPool';
import { ChainId } from '../components/MetamaskConnect';

export default class RewardPoolClient {
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

  async adminSetMarketplace(newAddress: string) {
    const contract = this.getContract();
    return contract.methods
      .adminSetMarketplace(newAddress)
      .send({ from: this.address });
  }

  async marketplace(): Promise<string> {
    const contract = this.getContract();
    return contract.methods.marketplace().call();
  }

  async availableReward(): Promise<bigint> {
    const contract = this.getContract();
    return contract.methods.availableReward().call();
  }

  async reservedAmount(): Promise<bigint> {
    const contract = this.getContract();
    return contract.methods.reservedAmount().call();
  }

  private getContract() {
    return new this.web3.eth.Contract(ABI, CONTRACT_ADDRESS[this.chainId]);
  }
}
