import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import EkokeClient from '../../../../web3/EkokeClient';
import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';
import { convertToHumanReadable } from '../../../../utils/format';

const BalanceOf = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [address, setAddress] = React.useState<string>('');

  const onAddressChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAddress(event.target.value);
  };

  const onSubmit = () => {
    const client = new EkokeClient(account, ethereum, chainId as ChainId);

    if (!address) {
      alert('Address is required');
      return;
    }

    setPendingTx(true);

    client
      .balanceOf(address)
      .then((balance) => {
        const balanceHuman = convertToHumanReadable(balance);
        alert(`Balance of ${address}: ${balanceHuman}`);
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
        id="balance-of-address"
        value={address}
        onChange={onAddressChange}
        label="Balance of address"
      />
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Get balance of
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default BalanceOf;
