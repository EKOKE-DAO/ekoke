import * as React from 'react';
import { useMetaMask } from 'metamask-react';

import Container from '../reusable/Container';
import Card from '../reusable/Card';
import Alerts from '../reusable/Alerts';
import ChangeOwnerForm from './ChangeOwner/ChangeOwnerForm';
import Balance from './Balance';
import TransferForm from './Transfer/TransferForm';
import RenounceOwnershipForm from './RenounceOwnership/RenounceOwnershipForm';

const Form = () => {
  const { status } = useMetaMask();

  const content =
    status === 'connected' ? (
      <Container.FlexCols className="gap-8">
        <Balance />
        <Card>
          <TransferForm />
        </Card>
        <Card>
          <ChangeOwnerForm />
        </Card>
        <Card>
          <RenounceOwnershipForm />
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
