import { WalletProvider } from 'react-ic-wallet';

interface UserPreferences {
  icWallet?: WalletProvider;
}

const getPreferences = (): UserPreferences | null => {
  try {
    const storedPreferences = localStorage.getItem('userPreferences');
    return storedPreferences
      ? (JSON.parse(storedPreferences) as UserPreferences)
      : null;
  } catch (error) {
    console.error('Failed to fetch from local storage:', error);
    return null;
  }
};

export const getUserIcWallet = (): WalletProvider | undefined => {
  const preferences = getPreferences();
  return preferences?.icWallet;
};

export const setUserIcWallet = (icWallet: WalletProvider | undefined): void => {
  const preferences = getPreferences() || {};
  preferences.icWallet = icWallet;
  localStorage.setItem('userPreferences', JSON.stringify(preferences));
};
