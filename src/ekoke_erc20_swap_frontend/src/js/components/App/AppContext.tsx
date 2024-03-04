import * as React from 'react';
import { WalletProvider } from 'react-ic-wallet';
import { getUserIcWallet } from '../../storage';

interface Context {
  icWallet?: WalletProvider;
  setIcWallet?: (icWallet: WalletProvider | undefined) => void;
}

const AppContext = React.createContext<Context>({});

const AppContextProvider = ({ children }: { children?: React.ReactNode }) => {
  const [icWallet, setIcWallet] = React.useState<WalletProvider | undefined>();

  React.useEffect(() => {
    const userWallet = getUserIcWallet();
    if (userWallet) {
      setIcWallet(userWallet);
    }
  }, []);

  return (
    <AppContext.Provider value={{ icWallet, setIcWallet }}>
      {children}
    </AppContext.Provider>
  );
};

export default AppContextProvider;

export const useAppContext = () => React.useContext(AppContext);
