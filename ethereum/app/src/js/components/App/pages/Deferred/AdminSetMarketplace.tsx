import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import RewardPoolClient from '../../../../web3/RewardPoolClient';
import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';

const AdminSetMarketplace = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [address, setAddress] = React.useState<string>('');

  const onAddressChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAddress(event.target.value);
  };

  const onSubmit = () => {
    const client = new RewardPoolClient(account, ethereum, chainId as ChainId);

    if (!address) {
      alert('Address is required');
      return;
    }

    setPendingTx(true);

    client
      .adminSetMarketplace(address)
      .then(() => {
        alert(`Set Marketplace address to ${address}`);
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
        id="admin-reward-marketplace-address"
        value={address}
        onChange={onAddressChange}
        label="Marketplace Address"
      />
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Set marketplace
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default AdminSetMarketplace;
