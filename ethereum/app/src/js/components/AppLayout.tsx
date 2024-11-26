import * as React from 'react';
import { BrowserRouter } from 'react-router-dom';

import Container from './reusable/Container';
import Page from './reusable/Page';
import AppRouter from './App/Routes';
import Header from './Header';

const AppLayout = () => (
  <Page.BlankPage>
    <Header />
    <Container.PageContent className="py-8">
      <BrowserRouter>
        <AppRouter />
      </BrowserRouter>
    </Container.PageContent>
  </Page.BlankPage>
);

export default AppLayout;
