import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import EkokeClient from '../../../web3/EkokeClient';
import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import AdminMint from './Ekoke/AdminMint';
import AdminRewardPool from './Ekoke/AdminRewardPool';

const DECIMALS = 8;

const Ekoke = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [rewardPoolMintedSupply, setRewardPoolMintedSupply] =
    React.useState<BigInt>();
  const [totalSupply, setTotalSupply] = React.useState<BigInt>();
  const [rewardPool, setRewardPool] = React.useState<string>();
  const [balance, setBalance] = React.useState<string>();

  React.useEffect(() => {
    const client = new EkokeClient(account, ethereum, chainId as ChainId);

    client.rewardPoolMintedSupply().then(setRewardPoolMintedSupply);
    client.rewardPool().then(setRewardPool);
    client.totalSupply().then(setTotalSupply);
    client.balanceOf(account).then((accountBalance) => {
      // put comma in `decimals` position
      const balanceStr = accountBalance.toString();
      const balanceArr = balanceStr.split('');
      balanceArr.splice(balanceArr.length - DECIMALS, 0, ',');
      console.log(balanceArr);
      setBalance(balanceArr.join(''));
    });
  }, []);

  return (
    <Container.FlexCols className="gap-4">
      <Container.Container>
        <span className="block">Total Supply: {totalSupply?.toString()}</span>
        <span className="block">Reward Pool: {rewardPool}</span>
        <span className="block">
          Reward minted supply: {rewardPoolMintedSupply?.toString()}
        </span>
        <span className="block">
          Balance: <strong>{balance?.toString()}</strong> EKOKE
        </span>
      </Container.Container>
      <Container.FlexCols className="gap-8 w-3/6">
        <AdminMint />
        <AdminRewardPool />
      </Container.FlexCols>
    </Container.FlexCols>
  );
};

export default Ekoke;
