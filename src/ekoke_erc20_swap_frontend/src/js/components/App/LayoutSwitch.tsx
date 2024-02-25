import * as React from 'react';
import { useMetaMask } from 'metamask-react';
import { useIcWallet } from 'react-ic-wallet';

import ConnectMessage from './ConnectMessage';
import ConnectedPage from './ConnectedPage';

const LayoutSwitch = () => {
  const { status: metamaskStatus } = useMetaMask();
  const { status: icStatus } = useIcWallet();

  if (metamaskStatus === 'connected' && icStatus === 'connected') {
    return <ConnectedPage />;
  }

  // default message to connect
  return <ConnectMessage />;
};

export default LayoutSwitch;
