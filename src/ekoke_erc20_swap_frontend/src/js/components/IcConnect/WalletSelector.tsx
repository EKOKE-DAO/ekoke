import * as React from 'react';
import { WalletProvider } from 'react-ic-wallet';

import Container from '../reusable/Container';
import Button from '../reusable/Button';

import IcpWalletLogo from '../../../assets/images/icp-wallet.webp';
import BitfinityWalletLogo from '../../../assets/images/bitfinity-wallet.webp';
import PlugWalletLogo from '../../../assets/images/plug-wallet.webp';

const WalletSelector = ({
  onSelect,
}: {
  onSelect: (wallet: WalletProvider) => void;
}) => {
  return (
    <Container.Container className="h-screen left-0 overflow-hidden fixed right-0 top-0 w-screen z-50">
      <Container.Container className="bg-gray-800/60 h-screen w-screen" />
      <Container.Container className="bg-white bottom-0 h-fit sm:h-[70vh] sm:rounded-t-xl left-0 m-auto p-8 fixed right-0 top-0 sm:top-auto sm:bottom-0 w-fit min-w-[25%] sm:w-full">
        <Container.FlexCols className="gap-4 text-lg p-4">
          <span className="text-lg text-center block text-text">
            Connect a wallet
          </span>
          <Container.Container className="grid grid-cols-2 sm:grid-cols-1 gap-4 px-4">
            <Wallet
              logo={IcpWalletLogo}
              name="Internet Identity"
              onSelect={onSelect}
              wallet={WalletProvider.Dfinity}
            />
            <Wallet
              logo={BitfinityWalletLogo}
              name="Bitfinity Wallet"
              onSelect={onSelect}
              wallet={WalletProvider.Bitfinity}
            />
            <Wallet
              logo={PlugWalletLogo}
              name="Plug"
              onSelect={onSelect}
              wallet={WalletProvider.Plug}
            />
          </Container.Container>
        </Container.FlexCols>
      </Container.Container>
    </Container.Container>
  );
};

const Wallet = ({
  logo,
  name,
  onSelect,
  wallet,
}: {
  logo: string;
  name: string;
  onSelect: (wallet: WalletProvider) => void;
  wallet: WalletProvider;
}) => (
  <Container.Container>
    <Button.Alternative className="w-full" onClick={() => onSelect(wallet)}>
      <Container.FlexRow className="items-center justify-between w-full">
        <span className="text-lg text-text">{name}</span>
        <img
          className="rounded-full bg-white border border-gray-300 p-1 w-[48px] h-[48px]"
          src={logo}
          alt={name}
          width={64}
          height={64}
        />
      </Container.FlexRow>
    </Button.Alternative>
  </Container.Container>
);

export default WalletSelector;
