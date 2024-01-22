import * as React from 'react';
import { useConnectedMetaMask, useMetaMask } from 'metamask-react';
import Web3Client from '../../web3/Web3Client';
import Container from '../reusable/Container';
import Heading from '../reusable/Heading';
import { ChainId } from '../MetamaskConnect';

const SwappedSupply = ({ decimals }: { decimals: number }) => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [swappedSupply, setSwappedSupply] = React.useState<string>('0');

  React.useEffect(() => {
    const client = new Web3Client(account, ethereum, chainId as ChainId);
    client.swappedSupply().then((supply) => {
      // put comma in `decimals` position
      const supplyStr = supply.toString();
      const arr = supplyStr.split('');
      arr.splice(arr.length - decimals, 0, ',');
      console.log(arr);
      setSwappedSupply(arr.join(''));
    });
  }, [decimals]);

  return (
    <Container.Container>
      <Heading.H2>
        Total circulating supply (swapped): {swappedSupply}
      </Heading.H2>
    </Container.Container>
  );
};

export default SwappedSupply;
