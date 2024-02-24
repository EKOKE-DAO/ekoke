import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import Web3Client from '../../../web3/Web3Client';
import { ChainId } from '../../MetamaskConnect';
import { e8sToEkoke } from '../../../utils';
import Container from '../../reusable/Container';
import Heading from '../../reusable/Heading';
import Hr from '../../reusable/Hr';
import Link from '../../reusable/Link';
import { CONTRACT_ADDRESS } from '../../../web3/contracts/Ekoke';
import Button from '../../reusable/Button';
import { Page, PageProps } from '../ConnectedPage';

const Summary = ({ onSwitchPage }: PageProps) => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [swappedSupply, setSwappedSupply] = React.useState<string>('');

  React.useEffect(() => {
    if (account && chainId) {
      const client = new Web3Client(account, ethereum, chainId as ChainId);
      client.swappedSupply().then((supply) => {
        setSwappedSupply(e8sToEkoke(supply));
      });
    }
  }, [account, chainId]);

  return (
    <Container.Card>
      <Heading.H1>EKOKE Token</Heading.H1>
      <Hr />
      <Container.FlexCols>
        <Container.Container>
          <span className="text-xl">
            ERC20 Token Address:{' '}
            <Link.Default
              href={`https://etherscan.io/address/${
                CONTRACT_ADDRESS[chainId as ChainId]
              }`}
            >
              {CONTRACT_ADDRESS[chainId as ChainId]}
            </Link.Default>
          </span>
        </Container.Container>
        <Container.Container>
          <span className="text-xl">ERC20 Swapped Supply: {swappedSupply}</span>
        </Container.Container>
        <Container.FlexResponsiveRow className="items-center justify-center gap-8 sm:gap-2">
          <Button.Alternative onClick={() => onSwitchPage(Page.IcrcToErc20)}>
            <span>Swap ICRC into ERC20</span>
          </Button.Alternative>
          <Button.Alternative onClick={() => onSwitchPage(Page.Erc20ToIcrc)}>
            <span>Swap ERC20 into ICRC</span>
          </Button.Alternative>
        </Container.FlexResponsiveRow>
      </Container.FlexCols>
    </Container.Card>
  );
};

export default Summary;
