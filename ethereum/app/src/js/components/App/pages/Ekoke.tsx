import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import EkokeClient from '../../../web3/EkokeClient';
import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import AdminMint from './Ekoke/AdminMint';
import AdminRewardPool from './Ekoke/AdminRewardPool';

const DECIMALS = 8;

const convertToHumanReadable = (value: BigInt) => {
  // put comma in `decimals` position
  const balanceStr = value.toString();
  const balanceArr = balanceStr.split('');
  balanceArr.splice(balanceArr.length - DECIMALS, 0, ',');
  console.log(balanceArr);
  return balanceArr.join('');
};

const Ekoke = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [rewardPoolMintedSupply, setRewardPoolMintedSupply] =
    React.useState<string>();
  const [totalSupply, setTotalSupply] = React.useState<string>();
  const [rewardPool, setRewardPool] = React.useState<string>();
  const [balance, setBalance] = React.useState<string>();
  const [ownerMintedSupply, setOwnerMintedSupply] = React.useState<string>();
  const [maxOwnerMintedSupply, setMaxOwnerMintedSupply] =
    React.useState<string>();

  React.useEffect(() => {
    const client = new EkokeClient(account, ethereum, chainId as ChainId);

    client.maxOwnerMintedSupply().then((supply) => {
      setMaxOwnerMintedSupply(convertToHumanReadable(supply));
    });
    client.ownerMintedSupply().then((supply) => {
      setOwnerMintedSupply(convertToHumanReadable(supply));
    });
    client.rewardPoolMintedSupply().then((supply) => {
      setRewardPoolMintedSupply(convertToHumanReadable(supply));
    });
    client.rewardPool().then(setRewardPool);
    client.totalSupply().then((supply) => {
      setTotalSupply(convertToHumanReadable(supply));
    });
    client.balanceOf(account).then((accountBalance) => {
      setBalance(convertToHumanReadable(accountBalance));
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
          Owner minted supply: {ownerMintedSupply?.toString()} /{' '}
          {maxOwnerMintedSupply?.toString()}
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
