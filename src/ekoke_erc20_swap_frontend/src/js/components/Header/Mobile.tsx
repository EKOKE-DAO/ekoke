import * as React from 'react';

import Container from '../reusable/Container';
import EkokeLogo from '../../../assets/images/ekoke-logo.webp';

const Mobile = () => (
  <div className="h-[80px] items-center bg-white hidden sm:flex left-0 gap-4 justify-start py-2 px-4 absolute top-0 w-full z-40 shadow-lg">
    <Container.FlexRow className="items-center">
      <img src={EkokeLogo} alt="Ekoke Logo" className="h-[64px] mr-4" />
      <span className="text-xl sm:text-lg text-brand">EKOKE ERC20 Swap</span>
    </Container.FlexRow>
  </div>
);

export default Mobile;
