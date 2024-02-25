import * as React from 'react';
import Summary from './pages/Summary';

import SwapErc20ToIcrc from './pages/SwapErc20ToIcrc';

export enum Page {
  Summary,
  IcrcToErc20,
  Erc20ToIcrc,
}

export interface PageProps {
  onSwitchPage: (page: Page) => void;
}

const ConnectedPage = () => {
  const [page, setPage] = React.useState<Page>(Page.Summary);

  if (page === Page.Erc20ToIcrc) {
    return <SwapErc20ToIcrc onSwitchPage={setPage} />;
  }

  return <Summary onSwitchPage={setPage} />;
};

export default ConnectedPage;
