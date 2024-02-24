import * as React from 'react';

import Container from '../reusable/Container';
import EkokeLogo from '../../../assets/images/ekoke-logo.webp';
import MetamaskConnect from '../MetamaskConnect';
import IcConnect from '../IcConnect';
import Paragraph from '../reusable/Paragraph';

const ConnectMessage = () => (
  <Container.Card>
    <Container.FlexCols className="justify-between items-center gap-4">
      <Container.FlexCols className="justify-between items-center">
        <img src={EkokeLogo} alt="Ekoke Logo" className="h-[64px] mr-4" />
        <span className="text-xl text-brand">EKOKE ERC20 Swap</span>
        <Paragraph.Leading className="!text-center">
          Welcome to the EKOKE ERC20 Swap.
          <br />
          This page allows you to swap your EKOKE ICRC Tokens into ERC20 EKOKE
          tokens and viceversa.
        </Paragraph.Leading>
        <Paragraph.Default className="!text-center">
          Please connect to both Metamask and IC Wallet to continue
        </Paragraph.Default>
      </Container.FlexCols>
      <Container.FlexRow className="items-center">
        <MetamaskConnect />
        <IcConnect />
      </Container.FlexRow>
    </Container.FlexCols>
  </Container.Card>
);

export default ConnectMessage;
