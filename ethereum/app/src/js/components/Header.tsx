import * as React from 'react';

import Container from './reusable/Container';

import { Route } from '../utils/routes';
import TopbarLink from './Header/TopbarLink';
import MetamaskConnect from './MetamaskConnect';

const Header = () => (
  <div className="fixed bg-white block left-0 top-0 h-[100px] w-full bg-page z-40 shadow-sm">
    <Container.FlexRow className="justify-center items-center py-4 px-4">
      <Container.FlexRow className="items-center gap-8">
        <TopbarLink name={'Home'} href={Route.HOME} />
        <TopbarLink name={'Deferred'} href={Route.DEFERRED} />
        <TopbarLink name={'EKOKE'} href={Route.EKOKE} />
        <TopbarLink name={'Presale'} href={Route.EKOKE_PRESALE} />
        <TopbarLink name={'Marketplace'} href={Route.MARKETPLACE} />
        <TopbarLink name={'Reward Pool'} href={Route.REWARD_POOL} />
        <MetamaskConnect />
      </Container.FlexRow>
    </Container.FlexRow>
  </div>
);

export default Header;
