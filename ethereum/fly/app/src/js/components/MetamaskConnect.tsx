import * as React from 'react';
import { useMetaMask } from 'metamask-react';
import Logo from './MetamaskConnect/Logo';
import Button from './reusable/Button';
import Container from './reusable/Container';
import Select from './reusable/Select';

export enum ChainId {
  Mainnet = '0x1',
  Ropsten = '0x3',
  Rinkeby = '0x4',
  Goerli = '0x5',
  Kovan = '0x2a',
  Hardhat = '0x7a69',
}

const MetamaskConnect = () => {
  const {
    status,
    connect,
    account,
    chainId: currentChainId,
    switchChain,
  } = useMetaMask();
  const [chainId, setChainId] = React.useState<ChainId>(
    currentChainId as ChainId,
  );

  const disabled =
    ['initializing', 'unavailable', 'connecting', 'connected'].includes(
      status,
    ) || !chainId;

  const onClick = () => {
    if (status === 'notConnected' && chainId) {
      if (chainId !== currentChainId) {
        switchChain(chainId);
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

  React.useEffect(() => {
    if (currentChainId && chainId && currentChainId !== chainId) {
      switchChain(chainId);
    }
  }, [chainId, switchChain]);

  React.useEffect(() => {
    if (!chainId) {
      setChainId(currentChainId as ChainId);
    }
  }, [currentChainId]);

  return (
    <Container.FlexRow className="items-center gap-8">
      <Select
        id="metamask-chainid-select"
        label="Network"
        className="text-white text"
        value={chainId}
        onChange={(e) => setChainId(e.target.value as ChainId)}
      >
        {!chainId && (
          <option value="" selected>
            Select a network
          </option>
        )}
        <option value={ChainId.Mainnet}>Mainnet</option>
        <option value={ChainId.Goerli}>Goerli</option>
        <option value={ChainId.Hardhat}>Hardhat</option>
      </Select>
      <Button.Alternative
        className="my-0 !mb-0"
        onClick={onClick}
        disabled={disabled}
      >
        <Logo />
        {text()}
      </Button.Alternative>
    </Container.FlexRow>
  );
};

export default MetamaskConnect;
