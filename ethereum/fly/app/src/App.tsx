import * as React from 'react';
import { MetaMaskProvider } from 'metamask-react';
import AppLayout from './js/components/AppLayout';

const App = () => (
  <MetaMaskProvider>
    <AppLayout />
  </MetaMaskProvider>
);

export default App;
