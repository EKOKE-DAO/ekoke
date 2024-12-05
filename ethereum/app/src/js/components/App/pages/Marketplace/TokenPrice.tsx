import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import MarketplaceClient from '../../../../web3/MarketplaceClient';
import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';

const TokenPrice = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [contractId, setContractId] = React.useState<string>('');

  const onContractIdChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setContractId(event.target.value);
  };

  const onSubmit = () => {
    const client = new MarketplaceClient(account, ethereum, chainId as ChainId);

    if (!contractId) {
      alert('Address is required');
      return;
    }

    setPendingTx(true);

    client
      .tokenPriceForCaller(BigInt(contractId))
      .then((price) => {
        const usdPrice = price / BigInt(1_000_000);
        alert(`Token price for caller: ${usdPrice.toString()} USD`);
        setContractId('');
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
        id="token-price-contract"
        value={contractId}
        onChange={onContractIdChange}
        label="Contract id"
      />
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Get token price
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default TokenPrice;
