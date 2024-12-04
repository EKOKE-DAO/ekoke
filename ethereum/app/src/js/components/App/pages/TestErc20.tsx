import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import TestErc20Client from '../../../web3/TestErc20Client';
import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import AdminMint from './TestErc20/AdminMint';
import { convertToHumanReadable } from '../../../utils/format';
import Approve from './TestErc20/Approve';

const TestERC20 = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [balance, setBalance] = React.useState<string>();

  React.useEffect(() => {
    const client = new TestErc20Client(account, ethereum, chainId as ChainId);

    client.balanceOf(account).then((accountBalance) => {
      setBalance(convertToHumanReadable(accountBalance, 6));
    });
  }, []);

  return (
    <Container.FlexCols className="gap-4">
      <Container.Container>
        <span className="block">
          Balance: <strong>{balance?.toString()}</strong> USDT
        </span>
      </Container.Container>
      <Container.FlexCols className="gap-8 w-3/6">
        <AdminMint />
        <Approve />
      </Container.FlexCols>
    </Container.FlexCols>
  );
};

export default TestERC20;
