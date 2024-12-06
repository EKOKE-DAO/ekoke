import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import MarketplaceClient from '../../../../web3/MarketplaceClient';
import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';
import Heading from '../../../reusable/Heading';

const AdminWithdraw = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [amount, setAmount] = React.useState<string>('');

  const onAmountChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAmount(event.target.value);
  };

  const onSubmit = () => {
    const client = new MarketplaceClient(account, ethereum, chainId as ChainId);

    if (!amount) {
      alert('Address is required');
      return;
    }

    setPendingTx(true);

    client
      .adminWithdraw(BigInt(amount))
      .then(() => {
        alert(`Withdrawn ${amount} from contract`);
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
      <Heading.H2>Withdraw liquidity from marketplace</Heading.H2>
      <Input.Input
        id="withdraw-amount"
        value={amount}
        onChange={onAmountChange}
        label="Withdraw amount"
        type="number"
        min={1}
      />
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Withdraw liquidity
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default AdminWithdraw;
