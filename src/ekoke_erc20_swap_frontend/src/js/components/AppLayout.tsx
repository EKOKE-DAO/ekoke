import * as React from 'react';

import Container from './reusable/Container';
import Page from './reusable/Page';
import MetamaskConnect from './MetamaskConnect';
import IcConnect from './IcConnect';

import EkokeLogo from '../../assets/images/ekoke-logo.webp';
import LayoutSwitch from './App/LayoutSwitch';

const AppLayout = () => (
  <Page.BlankPage>
    <Container.FlexRow className="justify-between items-center py-4 bg-white border-b border-gray-300 shadow-lg px-4">
      <Container.FlexRow className="items-center">
        <img src={EkokeLogo} alt="Ekoke Logo" className="h-[64px] mr-4" />
        <span className="text-xl text-brand">EKOKE ERC20 Swap</span>
      </Container.FlexRow>
      <Container.FlexRow className="items-center">
        <MetamaskConnect />
        <IcConnect />
      </Container.FlexRow>
    </Container.FlexRow>
    <Container.PageContent className="py-8">
      <LayoutSwitch />
    </Container.PageContent>
  </Page.BlankPage>
);

export default AppLayout;
