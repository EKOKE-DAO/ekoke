import * as React from 'react';

import Container from './reusable/Container';
import Page from './reusable/Page';
import LayoutSwitch from './App/LayoutSwitch';
import Header from './Header';
import Footer from './Footer';

const AppLayout = () => (
  <Page.BlankPage>
    <Header />
    <Container.PageContent className="py-8 min-h-[80vh]">
      <LayoutSwitch />
    </Container.PageContent>
    <Footer />
  </Page.BlankPage>
);

export default AppLayout;
