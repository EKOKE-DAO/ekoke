import * as React from 'react';
import { useConnectedMetaMask, useMetaMask } from 'metamask-react';
import Web3Client from '../../web3/Web3Client';
import Container from '../reusable/Container';
import Heading from '../reusable/Heading';
import { ChainId } from '../MetamaskConnect';

const SwappedSupply = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [swappedSupply, setSwappedSupply] = React.useState(0);

  React.useEffect(() => {
    const client = new Web3Client(account, ethereum, chainId as ChainId);
    client.swappedSupply().then((supply) => {
      setSwappedSupply(supply);
    });
  });

  return (
    <Container.Container>
      <Heading.H2>
        Total circulating supply (swapped): {swappedSupply}
      </Heading.H2>
    </Container.Container>
  );
};

export default SwappedSupply;
