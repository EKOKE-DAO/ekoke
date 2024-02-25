import Web3 from 'web3';

import { ABI, CONTRACT_ADDRESS } from './contracts/Ekoke';
import { ChainId } from '../components/MetamaskConnect';
import { Principal } from '@dfinity/principal';
import { principalToBytes32 } from '../utils';

export default class Web3Client {
  private address: string;
  private web3: Web3;
  private chainId: ChainId;

  constructor(address: string, web3: any, chainId: ChainId) {
    this.address = address;
    this.web3 = new Web3(web3);
    this.chainId = chainId;
  }

  async swap(recipient: Principal, amount: bigint) {
    const contract = this.getContract();
    return contract.methods
      .swap(principalToBytes32(recipient), amount)
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

  async swappedSupply(): Promise<BigInt> {
    const contract = this.getContract();
    return contract.methods.swappedSupply().call();
  }

  private getContract() {
    return new this.web3.eth.Contract(ABI, CONTRACT_ADDRESS[this.chainId]);
  }
}
