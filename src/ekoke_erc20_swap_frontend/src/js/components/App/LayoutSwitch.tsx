import * as React from 'react';
import { useMetaMask } from 'metamask-react';
import { useIcWallet } from 'react-ic-wallet';

import ConnectMessage from './ConnectMessage';
import ConnectedPage from './ConnectedPage';
import { useAppContext } from './AppContext';

const LayoutSwitch = () => {
  const { status: metamaskStatus } = useMetaMask();
  const { status: icStatus } = useIcWallet();
  const { icWallet } = useAppContext();

  if (metamaskStatus === 'connected' && icStatus === 'connected' && icWallet) {
    return <ConnectedPage />;
  }

  // default message to connect
  return <ConnectMessage />;
};

export default LayoutSwitch;
