import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import DeferredClient from '../../../web3/DeferredClient';
import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';

const Deferred = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [deferredMinter, setDeferredMinter] = React.useState<string>();
  const [marketplace, setMarketplace] = React.useState<string>();
  const [rewardPool, setRewardPool] = React.useState<string>();

  React.useEffect(() => {
    const client = new DeferredClient(account, ethereum, chainId as ChainId);

    client.deferredMinter().then(setDeferredMinter);
    client.marketplace().then(setMarketplace);
    client.rewardPool().then(setRewardPool);
  }, []);

  return (
    <Container.FlexCols>
      <Container.Container>
        <span className="block">Deferred Minter: {deferredMinter}</span>
        <span className="block">Marketplace: {marketplace}</span>
        <span className="block">Reward Pool: {rewardPool}</span>
      </Container.Container>
    </Container.FlexCols>
  );
};

export default Deferred;
