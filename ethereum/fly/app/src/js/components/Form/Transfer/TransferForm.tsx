import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import Container from '../../reusable/Container';
import Heading from '../../reusable/Heading';
import Input from '../../reusable/Input';
import Button from '../../reusable/Button';
import Web3Client from '../../../web3/Web3Client';
import Alerts from '../../reusable/Alerts';
import { ChainId } from '../../MetamaskConnect';

const TransferForm = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [recipientAddress, setRecipientAddress] = React.useState('');
  const [amount, setAmount] = React.useState('');
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [error, setError] = React.useState<string>();

  const onRecipientAddressChange = (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    setRecipientAddress(event.target.value);
  };

  const onAmountChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAmount(event.target.value);
  };

  const onMint = () => {
    setPendingTx(true);
    const client = new Web3Client(account, ethereum, chainId as ChainId);

    const amoutNum = Number(amount);

    client
      .transfer(recipientAddress, amoutNum)
      .then(() => {
        setPendingTx(false);
        setError(undefined);
      })
      .catch((e) => {
        setError(e.message);
        setPendingTx(false);
      });
  };

  const btnDisabled =
    !isAmountNumber(amount) || recipientAddress.length !== 42 || pendingTx;

  return (
    <Container.FlexCols className="items-center">
      <Heading.H2>Transfer tokens</Heading.H2>
      <Input.Input
        id="mint-form-recipient-address"
        label="Recipient address"
        onChange={onRecipientAddressChange}
        value={recipientAddress}
      />
      <Input.Input
        id="mint-form-token-amount"
        label="Token amount"
        onChange={onAmountChange}
        value={amount}
      />
      <Button.Primary disabled={btnDisabled} onClick={onMint} className="!mt-4">
        Transfer
      </Button.Primary>
      {error && (
        <Alerts.Danger>
          <p>{error}</p>
        </Alerts.Danger>
      )}
    </Container.FlexCols>
  );
};

const isAmountNumber = (amount: string) => {
  const amountNum = Number(amount);
  return !isNaN(amountNum);
};

export default TransferForm;
