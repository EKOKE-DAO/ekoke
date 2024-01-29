import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import Container from '../../reusable/Container';
import Heading from '../../reusable/Heading';
import Button from '../../reusable/Button';
import Web3Client from '../../../web3/Web3Client';
import Alerts from '../../reusable/Alerts';
import { ChainId } from '../../MetamaskConnect';

const RenounceOwnershipForm = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);
  const [error, setError] = React.useState<string>();

  const onTransfer = () => {
    setPendingTx(true);
    const client = new Web3Client(account, ethereum, chainId as ChainId);
    client
      .renounceOwnership()
      .then(() => {
        setPendingTx(false);
        setError(undefined);
      })
      .catch((e) => {
        setError(e.message);
        setPendingTx(false);
      });
  };

  const btnDisabled = pendingTx;

  return (
    <Container.FlexCols className="items-center">
      <Heading.H2>Renounce ownership</Heading.H2>
      <Button.Danger
        disabled={btnDisabled}
        onClick={onTransfer}
        className="!mt-4"
      >
        Renounce ownership
      </Button.Danger>
      {error && (
        <Alerts.Danger>
          <p>{error}</p>
        </Alerts.Danger>
      )}
    </Container.FlexCols>
  );
};

export default RenounceOwnershipForm;
