import * as React from 'react';

import { Route } from '../../../utils/routes';
import Container from '../../reusable/Container';
import Link from '../../reusable/Link';

const Home = () => (
  <Container.FlexCols className="gap-4">
    <Container.Container>
      <Link.Default href={Route.DEFERRED}>Deferred</Link.Default>
    </Container.Container>
    <Container.Container>
      <Link.Default href={Route.EKOKE}>EKOKE</Link.Default>
    </Container.Container>
    <Container.Container>
      <Link.Default href={Route.MARKETPLACE}>Marketplace</Link.Default>
    </Container.Container>
    <Container.Container>
      <Link.Default href={Route.REWARD_POOL}>RewardPool</Link.Default>
    </Container.Container>
  </Container.FlexCols>
);

export default Home;
