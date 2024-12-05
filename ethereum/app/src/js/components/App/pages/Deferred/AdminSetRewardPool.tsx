import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import DeferredClient from '../../../../web3/DeferredClient';
import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';

const AdminSetRewardPool = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [address, setAddress] = React.useState<string>('');

  const onAddressChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAddress(event.target.value);
  };

  const onSubmit = () => {
    const client = new DeferredClient(account, ethereum, chainId as ChainId);

    if (!address) {
      alert('Address is required');
      return;
    }

    setPendingTx(true);

    client
      .adminSetRewardPool(address)
      .then(() => {
        alert(`Set reward pool address to ${address}`);
        setAddress('');
        setPendingTx(false);
      })
      .catch((error) => {
        alert(`Error: ${error.message}`);
        setPendingTx(false);
      });
  };

  return (
    <Container.FlexCols>
      <Input.Input
        id="admin-reward-pool-address"
        value={address}
        onChange={onAddressChange}
        label="Reward Pool Address"
      />
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Set reward pool
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default AdminSetRewardPool;
