import * as React from 'react';

import Container from '../reusable/Container';
import MetamaskConnect from '../MetamaskConnect';
import IcConnect from '../IcConnect';

import EkokeLogo from '../../../assets/images/ekoke-logo.webp';
import Link from '../reusable/Link';

const Desktop = () => (
  <Container.FlexRow className="block sm:hidden justify-between items-center py-4 bg-white border-b border-gray-300 shadow-lg px-4">
    <Container.FlexRow className="items-center gap-8">
      <Container.FlexRow className="items-center">
        <img src={EkokeLogo} alt="Ekoke Logo" className="h-[64px] mr-4" />
        <span className="text-xl text-brand">EKOKE ERC20 Swap</span>
      </Container.FlexRow>
      <Link.Default
        href={'https://ekoketoken.com'}
        target="_blank"
        className="text-xl"
      >
        Token Website
      </Link.Default>
    </Container.FlexRow>
    <Container.FlexRow className="items-center">
      <MetamaskConnect />
      <IcConnect />
    </Container.FlexRow>
  </Container.FlexRow>
);

export default Desktop;
