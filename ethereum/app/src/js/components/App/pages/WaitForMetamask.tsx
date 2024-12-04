import { useMetaMask } from 'metamask-react';
import * as React from 'react';

interface Props {
  children: React.ReactNode | React.ReactNode[];
}

const WaitForMetamask = ({ children }: Props) => {
  const { status } = useMetaMask();

  if (status === 'connected') {
    return <>{children}</>;
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen">
      <div className="text-2xl text-center">
        Please connect your MetaMask wallet
      </div>
    </div>
  );
};

export default WaitForMetamask;
