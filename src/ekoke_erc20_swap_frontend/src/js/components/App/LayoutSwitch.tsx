import * as React from 'react';
import { useMetaMask } from 'metamask-react';
import { useIcWallet } from 'react-ic-wallet';

import ConnectMessage from './ConnectMessage';

const LayoutSwitch = () => {
  const { status: metamaskStatus } = useMetaMask();
  const { status: icStatus } = useIcWallet();

  if (metamaskStatus === 'connected' && icStatus === 'connected') {
    return (
      <div>
        <h1>Connected to both Metamask and IC Wallet</h1>
      </div>
    );
  }

  // default message to connect
  return <ConnectMessage />;
};

export default LayoutSwitch;
