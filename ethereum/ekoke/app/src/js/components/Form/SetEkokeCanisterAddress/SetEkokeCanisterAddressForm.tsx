import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import Container from '../../reusable/Container';
import Heading from '../../reusable/Heading';
import Input from '../../reusable/Input';
import Button from '../../reusable/Button';
import Web3Client from '../../../web3/Web3Client';
import Alerts from '../../reusable/Alerts';
import { ChainId } from '../../MetamaskConnect';

const setEkokeCanisterAddressForm = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [address, setAddress] = React.useState('');
  const [addressSet, setAddressSet] = React.useState<string | null>(null);
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [error, setError] = React.useState<string>();

  const onAddressChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAddress(event.target.value);
  };

  const onChangeAddress = () => {
    setPendingTx(true);
    const client = new Web3Client(account, ethereum, chainId as ChainId);
    client
      .setEkokeCanisterAddress(address)
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
    if (account && ethereum && chainId) {
      const client = new Web3Client(account, ethereum, chainId as ChainId);
      client
        .getEkokeCanisterAddress()
        .then((address) => {
          setAddressSet(address);
        })
        .catch((e) => {
          console.log('fly canister is probably unset', e);
          setAddressSet(null);
        });
    }
  }, [account, ethereum, chainId]);

  const btnDisabled = address.length != 42 || pendingTx;

  return (
    <Container.FlexCols className="items-center">
      {addressSet ? (
        <Heading.H2>Ekoke canister address: {addressSet}</Heading.H2>
      ) : (
        <>
          <Heading.H2>Set Ekoke canister address</Heading.H2>
          <Input.Input
            id="change-fly-canister-form-address"
            label="Ekoke canister ETH address"
            onChange={onAddressChange}
            value={address}
          />
          <Button.Danger
            disabled={btnDisabled}
            onClick={onChangeAddress}
            className="!mt-4"
          >
            Set fly canister address
          </Button.Danger>
          {error && (
            <Alerts.Danger>
              <p>{error}</p>
            </Alerts.Danger>
          )}
        </>
      )}
    </Container.FlexCols>
  );
};

export default setEkokeCanisterAddressForm;
