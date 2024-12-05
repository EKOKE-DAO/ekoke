import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import DeferredClient from '../../../../web3/DeferredClient';
import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';

const OwnerOf = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [tokenId, setTokenId] = React.useState<string>('');

  const onTokenIdChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setTokenId(event.target.value);
  };

  const onSubmit = () => {
    const client = new DeferredClient(account, ethereum, chainId as ChainId);

    if (!tokenId) {
      alert('Token id is required');
      return;
    }

    setPendingTx(true);
    const tokenIdNum = Number(tokenId);

    client
      .ownerOf(tokenIdNum)
      .then((address) => {
        alert(`Owner of ${tokenId}: ${address}`);
        setTokenId('');
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
        id="owner-of-token-id"
        value={tokenId}
        onChange={onTokenIdChange}
        label="Token ID"
        required
      />
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Get owner of token
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default OwnerOf;
