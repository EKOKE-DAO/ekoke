import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import TestErc20Client from '../../../../web3/TestErc20Client';
import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';

const Approve = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [amount, setAmount] = React.useState<string>('');
  const [address, setAddress] = React.useState<string>('');

  const onAddressChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAddress(event.target.value);
  };

  const onAmountChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAmount(event.target.value);
  };

  const onApprove = () => {
    const client = new TestErc20Client(account, ethereum, chainId as ChainId);

    if (!address) {
      alert('Address is required');
      return;
    }

    if (!amount) {
      alert('Amount is required');
      return;
    }

    setPendingTx(true);

    const amoutNumber = Number(amount);

    client
      .approve(address, amoutNumber)
      .then(() => {
        alert(`Approved ${amoutNumber} to ${address}`);
        setAddress('');
        setAmount('');
        setPendingTx(false);
      })
      .catch((error) => {
        alert(`Error: ${error.message}`);
        setPendingTx(false);
      });
  };

  return (
    <Container.FlexCols>
      <span className="block text-xl">Approve</span>
      <Input.Input
        id="admin-mint-recipient"
        value={address}
        onChange={onAddressChange}
        label="Recipient Address"
      />
      <Input.Input
        id="admin-mint-amount"
        value={amount}
        onChange={onAmountChange}
        label="Amount"
        type="number"
      />
      <Button.Primary disabled={pendingTx} onClick={onApprove}>
        Approve
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default Approve;
