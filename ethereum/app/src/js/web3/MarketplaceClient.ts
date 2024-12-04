import Web3 from 'web3';

import { ABI, CONTRACT_ADDRESS } from './contracts/Marketplace';
import { ChainId } from '../components/MetamaskConnect';

export default class MarketplaceClient {
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

  async adminSetInterestRate(rate: number) {
    const contract = this.getContract();
    return contract.methods
      .adminSetInterestRate(rate)
      .send({ from: this.address });
  }

  async adminSetRewardPool(newAddress: string) {
    const contract = this.getContract();
    return contract.methods
      .adminSetRewardPool(newAddress)
      .send({ from: this.address });
  }

  async interestRate(): Promise<number> {
    const contract = this.getContract();
    return contract.methods.interestRate().call();
  }

  async usdErc20(): Promise<string> {
    const contract = this.getContract();
    return contract.methods.usdErc20().call();
  }

  private getContract() {
    return new this.web3.eth.Contract(ABI, CONTRACT_ADDRESS[this.chainId]);
  }
}
