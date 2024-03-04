import * as React from 'react';
import { MetaMaskProvider } from 'metamask-react';
import { IcWalletProvider } from 'react-ic-wallet';

import AppLayout from './js/components/AppLayout';
import AppContextProvider, {
  useAppContext,
} from './js/components/App/AppContext';

const App = () => (
  <MetaMaskProvider>
    <AppContextProvider>
      <AppLayoutWrapper />
    </AppContextProvider>
  </MetaMaskProvider>
);

const AppLayoutWrapper = () => {
  const { icWallet } = useAppContext();

  return (
    <IcWalletProvider provider={icWallet}>
      <AppLayout />
    </IcWalletProvider>
  );
};

export default App;
