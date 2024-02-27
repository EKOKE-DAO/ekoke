import * as React from 'react';
import { useMetaMask } from 'metamask-react';
import Button from './reusable/Button';
import Container from './reusable/Container';
import Metamask from './svg/Metamask';

export enum ChainId {
  Mainnet = '0x1',
  Ropsten = '0x3',
  Rinkeby = '0x4',
  Goerli = '0x5',
  Kovan = '0x2a',
  Sepolia = '0xaa36a7',
  Hardhat = '0x7a69',
}

const CHAIN_ID = ChainId.Sepolia;

const MetamaskConnect = () => {
  const { status, connect, account, chainId, switchChain } = useMetaMask();

  const disabled = [
    'initializing',
    'unavailable',
    'connecting',
    'connected',
  ].includes(status);

  const onClick = () => {
    if (status === 'notConnected') {
      if (CHAIN_ID !== chainId) {
        switchChain(CHAIN_ID);
      }
      return connect();
    }
    return undefined;
  };

  const addressText = (address: string) => {
    return `${address.substring(0, 6)}...${address.substring(
      address.length - 4,
    )}`;
  };

  const text = () => {
    if (status === 'initializing') return 'Initializing...';
    if (status === 'unavailable') return 'MetaMask not available';
    if (status === 'notConnected') return 'Connect to MetaMask';
    if (status === 'connecting') return 'Connecting...';
    if (status === 'connected') return addressText(account);
    return undefined;
  };

  return (
    <Container.FlexRow className="items-center gap-8">
      <Button.Alternative
        className="my-0 !mb-0"
        onClick={onClick}
        disabled={disabled}
      >
        <Metamask />
        {text()}
      </Button.Alternative>
    </Container.FlexRow>
  );
};

export default MetamaskConnect;
