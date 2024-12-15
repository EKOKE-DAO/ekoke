import * as React from 'react';
import { useConnectedMetaMask } from 'metamask-react';

import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import RewardPoolClient from '../../../web3/RewardPoolClient';
import AdminSetMarketplace from './RewardPool/AdminSetMarketplace';
import { convertToHumanReadable } from '../../../utils/format';

const RewardPool = () => {
  const { account, ethereum, chainId } = useConnectedMetaMask();

  const [availableReward, setAvailableReward] = React.useState<string>();
  const [reservedAmount, setReservedAmount] = React.useState<string>();
  const [Marketplace, setMarketplace] = React.useState<string>();

  React.useEffect(() => {
    const client = new RewardPoolClient(account, ethereum, chainId as ChainId);

    client
      .availableReward()
      .then((reward) => setAvailableReward(convertToHumanReadable(reward)));
    client
      .reservedAmount()
      .then((reserved) => setReservedAmount(convertToHumanReadable(reserved)));
    client.marketplace().then(setMarketplace);
  }, []);

  return (
    <Container.FlexCols className="gap-4">
      <Container.Container>
        <span className="block">Available reward: {availableReward}</span>
        <span className="block">Reserved amount: {reservedAmount}</span>
        <span className="block">Marketplace: {Marketplace}</span>
      </Container.Container>
      <Container.FlexCols className="gap-8 w-3/6">
        <AdminSetMarketplace />
      </Container.FlexCols>
    </Container.FlexCols>
  );
};

export default RewardPool;
