import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import MarketplaceClient from '../../../web3/MarketplaceClient';
import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import AdminSetRewardPool from './Marketplace/AdminSetRewardPool';
import TokenPrice from './Marketplace/TokenPrice';
import { convertToHumanReadable } from '../../../utils/format';
import AdminWithdraw from './Marketplace/AdminWithdraw';

const Marketplace = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();

  const [usdErc20, setUsdErc20] = React.useState<string>();
  const [rewardPool, setRewardPool] = React.useState<string>();
  const [interestRate, setInterestRate] = React.useState<string>();
  const [liquidityWithdrawable, setLiquidityWithdrawable] =
    React.useState<string>();

  React.useEffect(() => {
    const client = new MarketplaceClient(account, ethereum, chainId as ChainId);

    client.usdErc20().then(setUsdErc20);
    client.rewardPool().then(setRewardPool);
    client.interestRate().then((rate) => {
      setInterestRate(rate.toString());
    });
    client
      .liquidityWithdrawable()
      .then((value) =>
        setLiquidityWithdrawable(convertToHumanReadable(value, 6)),
      );
  }, []);

  return (
    <Container.FlexCols className="gap-4">
      <Container.Container>
        <span className="block">USD ERC20: {usdErc20}</span>
        <span className="block">Reward Pool: {rewardPool}</span>
        <span className="block">Interest Rate: {interestRate}</span>
        <span className="block">
          Liquidity Withdrawable: {liquidityWithdrawable} USDT
        </span>
      </Container.Container>
      <Container.FlexCols className="gap-8 w-3/6">
        <AdminSetRewardPool />
        <AdminWithdraw />
        <TokenPrice />
      </Container.FlexCols>
    </Container.FlexCols>
  );
};

export default Marketplace;
