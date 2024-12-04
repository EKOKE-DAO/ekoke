import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import MarketplaceClient from '../../../web3/MarketplaceClient';

const Marketplace = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();

  React.useEffect(() => {
    const client = new MarketplaceClient(account, ethereum, chainId as ChainId);
  }, []);

  return <Container.FlexCols></Container.FlexCols>;
};

export default Marketplace;