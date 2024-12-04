import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import RewardPoolClient from '../../../web3/RewardPool';

const RewardPool = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();

  React.useEffect(() => {
    const client = new RewardPoolClient(account, ethereum, chainId as ChainId);
  }, []);

  return <Container.FlexCols></Container.FlexCols>;
};

export default RewardPool;
