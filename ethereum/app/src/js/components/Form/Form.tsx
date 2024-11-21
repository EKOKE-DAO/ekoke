import * as React from 'react';
import { useMetaMask } from 'metamask-react';

import Container from '../reusable/Container';
import Card from '../reusable/Card';
import Alerts from '../reusable/Alerts';
import ChangeOwnerForm from './ChangeOwner/ChangeOwnerForm';
import Balance from './Balance';
import TransferForm from './Transfer/TransferForm';
import RenounceOwnershipForm from './RenounceOwnership/RenounceOwnershipForm';
import SwappedSupply from './SwappedSupply';
import MintTestnetTokens from './MintTestnetTokens/MintTestnetTokens';
import SetEkokeCanisterAddressForm from './SetEkokeCanisterAddress/SetEkokeCanisterAddressForm';
import Web3Client from '../../web3/Web3Client';
import { ChainId } from '../MetamaskConnect';

const Form = () => {
  const { status, account, ethereum, chainId } = useMetaMask();

  const [decimals, setDecimals] = React.useState<number>(0);

  React.useEffect(() => {
    if (status === 'connected' && account && ethereum && chainId) {
      const client = new Web3Client(account, ethereum, chainId as ChainId);
      client
        .decimals()
        .then((decs) => {
          console.log('decimals', decs);
          setDecimals(Number(decs));
        })
        .catch((e) => {
          console.log('failed to get balance', e);
        });
    }
  }, [status, account, ethereum, chainId]);

  const content =
    status === 'connected' ? (
      <Container.FlexCols className="gap-8">
        <Balance decimals={decimals} />
        <SwappedSupply decimals={decimals} />
        <Card>
          <MintTestnetTokens />
        </Card>
        <Card>
          <TransferForm />
        </Card>
        <Card>
          <ChangeOwnerForm />
        </Card>
        <Card>
          <RenounceOwnershipForm />
        </Card>
        <Card>
          <SetEkokeCanisterAddressForm />
        </Card>
      </Container.FlexCols>
    ) : (
      <Container.Container>
        <Alerts.Warning>
          Connettiti a Metamask per accedere al form
        </Alerts.Warning>
      </Container.Container>
    );

  return <Container.Container>{content}</Container.Container>;
};

export default Form;
