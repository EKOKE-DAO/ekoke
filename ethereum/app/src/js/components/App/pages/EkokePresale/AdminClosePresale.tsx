import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import { ChainId } from '../../../MetamaskConnect';
import Container from '../../../reusable/Container';
import Button from '../../../reusable/Button';
import EkokePresaleClient from '../../../../web3/EkokePresaleClient';

const AdminClosePresale = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [pendingTx, setPendingTx] = React.useState<boolean>(false);

  const onSubmit = () => {
    const client = new EkokePresaleClient(
      account,
      ethereum,
      chainId as ChainId,
    );

    client
      .adminClosePresale()
      .then(() => {
        alert(`Presale closed`);
        setPendingTx(false);
      })
      .catch((error) => {
        alert(`Error: ${error.message}`);
        setPendingTx(false);
      });
  };

  return (
    <Container.FlexCols>
      <Button.Primary disabled={pendingTx} onClick={onSubmit}>
        Close presale
      </Button.Primary>
    </Container.FlexCols>
  );
};

export default AdminClosePresale;
