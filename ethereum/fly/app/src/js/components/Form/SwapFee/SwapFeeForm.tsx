import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import Container from '../../reusable/Container';
import Heading from '../../reusable/Heading';
import Input from '../../reusable/Input';
import Button from '../../reusable/Button';
import Web3Client from '../../../web3/Web3Client';
import Alerts from '../../reusable/Alerts';
import { ChainId } from '../../MetamaskConnect';

const SwapFeeForm = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [fee, setFee] = React.useState<string>('');
  const [currentFee, setCurrentFee] = React.useState<number>();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [error, setError] = React.useState<string>();

  const onFeeChanged = (event: React.ChangeEvent<HTMLInputElement>) => {
    setFee(event.target.value);
  };

  const onChangeFee = () => {
    setPendingTx(true);
    const client = new Web3Client(account, ethereum, chainId as ChainId);

    const feeNum = Number(fee);

    client
      .setSwapFee(feeNum)
      .then(() => {
        setPendingTx(false);
        setError(undefined);
      })
      .catch((e) => {
        setError(e.message);
        setPendingTx(false);
      });
  };

  React.useEffect(() => {
    if (!account || !ethereum || !chainId) {
      return;
    }

    const client = new Web3Client(account, ethereum, chainId as ChainId);
    client
      .swapFee()
      .then((currFee) => {
        setCurrentFee(Number(currFee));
      })
      .catch((e) => {
        setError(e.message);
      });
  }, [account, ethereum, chainId]);

  const btnDisabled = !isAmountNumber(fee) || pendingTx;

  return (
    <Container.FlexCols className="items-center">
      <Heading.H2>Swap Fee</Heading.H2>
      <span>Current Swap Fee: {currentFee}</span>
      <Input.Input
        id="swap-fee-form-fee"
        label="New Fee"
        type="number"
        onChange={onFeeChanged}
        value={fee}
      />
      <Button.Primary
        disabled={btnDisabled}
        onClick={onChangeFee}
        className="!mt-4"
      >
        Change swap fee
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

export default SwapFeeForm;
