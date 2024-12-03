import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';
import EkokePresaleClient from '../../../../web3/EkokePresaleClient';
import { convertToHumanReadable } from '../../../../utils/format';

const ETH_DECIMALS = 18;

const Buy = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [amount, setAmount] = React.useState<string>('');
  const [tokenPrice, setTokenPrice] = React.useState<bigint>();
  const [ethPrice, setEthPrice] = React.useState<bigint>();

  const onAmountChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const amount = event.target.value;
    setAmount(amount);
    if (tokenPrice) {
      const ethPrice = BigInt(amount) * tokenPrice;
      setEthPrice(ethPrice);
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

    const amoutNumber = Number(amount);
    const tokenPriceNumber: number = Number(tokenPrice);
    const ethPriceNumber = amoutNumber * tokenPriceNumber;

    client
      .buyTokens(amoutNumber, ethPriceNumber)
      .then(() => {
        alert(`Bought ${amoutNumber} tokens for ${ethPriceNumber} ETH`);
        setAmount('');
        setEthPrice(undefined);
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
        {tokenPrice && convertToHumanReadable(tokenPrice, ETH_DECIMALS)} ETH
      </span>
      <span className="text-text">
        You pay: {ethPrice && convertToHumanReadable(ethPrice, ETH_DECIMALS)}{' '}
        ETH
      </span>
      <Input.Input
        id="admin-mint-amount"
        value={amount}
        onChange={onAmountChange}
        label="Amount"
        type="number"
      />
      <Button.Primary disabled={pendingTx} onClick={onBuy}>
        Mint tokens
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default Buy;
