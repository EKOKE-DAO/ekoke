import Web3 from 'web3';

import { ABI, CONTRACT_ADDRESS } from './contracts/Ekoke';
import { ChainId } from '../components/MetamaskConnect';

export default class EkokeClient {
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

  async adminSetRewardPoolAddress(newAddress: string) {
    const contract = this.getContract();
    return contract.methods
      .adminSetRewardPoolAddress(newAddress)
      .send({ from: this.address });
  }

  async rewardPool(): Promise<string> {
    const contract = this.getContract();
    return contract.methods.rewardPool().call();
  }

  async rewardPoolMintedSupply(): Promise<BigInt> {
    const contract = this.getContract();
    return contract.methods.rewardPoolMintedSupply().call();
  }

  async transfer(recipient: string, amount: number) {
    const contract = this.getContract();
    return contract.methods
      .transfer(recipient, amount)
      .send({ from: this.address });
  }

  async adminMint(recipient: string, amount: number) {
    const contract = this.getContract();
    return contract.methods
      .adminMint(recipient, amount)
      .send({ from: this.address });
  }

  async balanceOf(address: string): Promise<BigInt> {
    const contract = this.getContract();
    return contract.methods.balanceOf(address).call();
  }

  async decimals(): Promise<BigInt> {
    const contract = this.getContract();
    return contract.methods.decimals().call();
  }

  async totalSupply(): Promise<BigInt> {
    const contract = this.getContract();
    return contract.methods.totalSupply().call();
  }

  private getContract() {
    return new this.web3.eth.Contract(ABI, CONTRACT_ADDRESS[this.chainId]);
  }
}
