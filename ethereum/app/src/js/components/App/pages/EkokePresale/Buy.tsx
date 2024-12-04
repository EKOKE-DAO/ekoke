import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';
import EkokePresaleClient from '../../../../web3/EkokePresaleClient';
import { convertToHumanReadable } from '../../../../utils/format';

const USDT_DECIMALS = 6;

const Buy = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [amount, setAmount] = React.useState<string>('');
  const [tokenPrice, setTokenPrice] = React.useState<bigint>();
  const [usdPrice, setUsdPrice] = React.useState<bigint>();

  const onAmountChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const amount = event.target.value;
    setAmount(amount);
    if (tokenPrice) {
      const currentUsdPrice = BigInt(amount) * tokenPrice;
      setUsdPrice(currentUsdPrice);
    }
  };

  const onBuy = () => {
    const client = new EkokePresaleClient(
      account,
      ethereum,
      chainId as ChainId,
    );

    if (!amount) {
      alert('Amount is required');
      return;
    }

    setPendingTx(true);

    const amoutNumber: number = Number(amount);
    const tokenPriceNumber: number = Number(tokenPrice);

    client
      .buyTokens(amoutNumber)
      .then(() => {
        alert(
          `Bought ${amoutNumber} tokens for ${
            amoutNumber * tokenPriceNumber
          } USD`,
        );
        setAmount('');
        setUsdPrice(undefined);
        setPendingTx(false);
      })
      .catch((error) => {
        alert(`Error: ${error.message}`);
        setPendingTx(false);
      });
  };

  React.useEffect(() => {
    const client = new EkokePresaleClient(
      account,
      ethereum,
      chainId as ChainId,
    );

    client.tokenPrice().then((price) => {
      console.log('Token price', price);
      setTokenPrice(price);
    });
  }, []);

  return (
    <Container.FlexCols>
      <span className="text-text">
        Current token price:{' '}
        {tokenPrice !== undefined &&
          convertToHumanReadable(tokenPrice, USDT_DECIMALS, true)}{' '}
        USDT
      </span>
      <span className="text-text">
        You pay:{' '}
        {usdPrice !== undefined &&
          convertToHumanReadable(usdPrice, USDT_DECIMALS, true)}{' '}
        USDT
      </span>
      <Input.Input
        id="admin-mint-amount"
        value={amount}
        onChange={onAmountChange}
        label="Amount"
        type="number"
      />
      <Button.Primary disabled={pendingTx} onClick={onBuy}>
        Buy tokens
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default Buy;
