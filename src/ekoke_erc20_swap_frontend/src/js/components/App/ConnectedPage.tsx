import * as React from 'react';
import Summary from './pages/Summary';

import SwapErc20ToIcrc from './pages/SwapErc20ToIcrc';
import SwapIcrcToErc20 from './pages/SwapIcrcToErc20';
import AgentContextProvider from '../../ic/AgentContext';

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
  if (page === Page.IcrcToErc20) {
    return (
      <AgentContextProvider>
        <SwapIcrcToErc20 onSwitchPage={setPage} />
      </AgentContextProvider>
    );
  }

  return <Summary onSwitchPage={setPage} />;
};

export default ConnectedPage;
