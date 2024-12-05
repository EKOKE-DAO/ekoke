import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';
import DeferredClient from '../../../../web3/DeferredClient';
import { ChainId } from '../../../MetamaskConnect';
import { Contract } from 'web3';
import Input from '../../../reusable/Input';
import Button from '../../../reusable/Button';
import Heading from '../../../reusable/Heading';

const CreateContract = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);

  const [contractId, setContractId] = React.useState<string>();
  const [metadataUri, setMetadataUri] = React.useState<string>();
  const [seller, setSeller] = React.useState<string>();
  const [buyer, setBuyer] = React.useState<string>();
  const [reward, setReward] = React.useState<string>();
  const [priceUsd, setPriceUsd] = React.useState<string>();
  const [tokensAmount, setTokensAmount] = React.useState<string>();

  const onContractIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setContractId(e.target.value);
  };

  const onMetadataUriChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setMetadataUri(e.target.value);
  };

  const onSellerChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSeller(e.target.value);
  };

  const onBuyerChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setBuyer(e.target.value);
  };

  const onRewardChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setReward(e.target.value);
  };

  const onPriceUsdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPriceUsd(e.target.value);
  };

  const onTokensAmountChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setTokensAmount(e.target.value);
  };

  const onSubmit = () => {
    const client = new DeferredClient(account, ethereum, chainId as ChainId);

    if (!contractId) {
      alert('contractId is required');
      return;
    }

    if (!metadataUri) {
      alert('metadataUri is required');
      return;
    }

    if (!seller) {
      alert('seller is required');
      return;
    }

    if (!buyer) {
      alert('buyer is required');
      return;
    }

    if (!reward) {
      alert('reward is required');
      return;
    }

    if (!priceUsd) {
      alert('priceUsd is required');
      return;
    }

    if (!tokensAmount) {
      alert('tokensAmount is required');
      return;
    }

    const tokenIdNumber = BigInt(contractId);
    const rewardNumber = BigInt(reward);
    const priceUsdNumber = BigInt(priceUsd);
    const tokensAmountNumber = BigInt(tokensAmount);

    setPendingTx(true);

    client
      .createContract({
        contractId: tokenIdNumber,
        sellers: [{ seller, quota: 100 }],
        buyers: [buyer],
        metadataUri,
        ekokeReward: rewardNumber,
        tokenPriceUsd: priceUsdNumber,
        tokensAmount: tokensAmountNumber,
      })
      .then(() => {
        alert(`Contract created with ID ${tokenIdNumber}`);
        setContractId('');
        setMetadataUri('');
        setSeller('');
        setBuyer('');
        setReward('');
        setPriceUsd('');
        setTokensAmount('');
        setPendingTx(false);
      })
      .catch((error) => {
        alert(`Error: ${error.message}`);
        setPendingTx(false);
      });
  };

  return (
    <form className="flex flex-col gap-4" onSubmit={onSubmit}>
      <Heading.H2>Create contract</Heading.H2>
      <Input.Input
        id="create-contract-id"
        value={contractId}
        onChange={onContractIdChange}
        label="ContractId"
        required
      />
      <Input.Input
        id="create-contract-metadata-uri"
        value={metadataUri}
        onChange={onMetadataUriChange}
        label="Metadata URI"
        required
      />
      <Input.Input
        id="create-contract-seller"
        value={seller}
        onChange={onSellerChange}
        label="Seller"
        required
      />
      <Input.Input
        id="create-contract-buyer"
        value={buyer}
        onChange={onBuyerChange}
        label="Buyer"
        required
      />
      <Input.Input
        id="create-contract-reward"
        value={reward}
        onChange={onRewardChange}
        label="Reward"
        required
      />
      <Input.Input
        id="create-contract-price-usd"
        value={priceUsd}
        onChange={onPriceUsdChange}
        label="Price USD"
        required
      />
      <Input.Input
        id="create-contract-tokens-amount"
        value={tokensAmount}
        onChange={onTokensAmountChange}
        label="Tokens Amount"
        required
      />
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Create contract
      </Button.Primary>
    </form>
  );
};

export default CreateContract;
