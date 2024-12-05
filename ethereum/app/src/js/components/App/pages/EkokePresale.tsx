import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import EkokePresaleClient from '../../../web3/EkokePresaleClient';
import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import AdminClosePresale from './EkokePresale/AdminClosePresale';
import AdminSetPresaleCap from './EkokePresale/AdminSetPresaleCap';
import { convertToHumanReadable } from '../../../utils/format';
import Buy from './EkokePresale/Buy';

const EkokePresale = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const [presaleCapValue, setPresaleCapValue] = React.useState<bigint>();
  const [presaleCap, setPresaleCap] = React.useState<string>();
  const [softCap, setSoftCap] = React.useState<string>();
  const [tokensSold, setTokensSold] = React.useState<string>();

  const [balance, setBalance] = React.useState<string>();

  React.useEffect(() => {
    const client = new EkokePresaleClient(
      account,
      ethereum,
      chainId as ChainId,
    );

    client.presaleCap().then((value) => {
      setPresaleCapValue(value);
      setPresaleCap(convertToHumanReadable(value, 8, true));
    });

    client.tokensSold().then((value) => {
      setTokensSold(convertToHumanReadable(value, 8, true));
    });

    client.balanceOf(account).then((accountBalance) => {
      setBalance(convertToHumanReadable(accountBalance, 8, true));
    });

    client.softCap().then((value) => {
      setSoftCap(convertToHumanReadable(value, 6, true));
    });
  }, []);

  return (
    <Container.FlexCols className="gap-4">
      <Container.Container>
        <span className="block">
          Presale Maximum cap: {presaleCap?.toString()}
        </span>
        <span className="block">
          Tokens Sold: {tokensSold}/{softCap?.toString()} (soft cap)
        </span>
        <span className="block">
          Balance: <strong>{balance?.toString()}</strong> EKOKE
        </span>
      </Container.Container>
      <Container.FlexCols className="gap-8 w-3/6">
        <Buy />
        {presaleCapValue == BigInt(0) && <AdminSetPresaleCap />}
        <AdminClosePresale />
      </Container.FlexCols>
    </Container.FlexCols>
  );
};

export default EkokePresale;
