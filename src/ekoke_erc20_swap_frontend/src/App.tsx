import * as React from 'react';
import { MetaMaskProvider } from 'metamask-react';
import { IcWalletProvider } from 'react-ic-wallet';

import AppLayout from './js/components/AppLayout';

const App = () => {
  return (
    <MetaMaskProvider>
      <IcWalletProvider>
        <AppLayout />
      </IcWalletProvider>
    </MetaMaskProvider>
  );
};

export default App;
