import * as React from 'react';
import Summary from './pages/Summary';

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

  return <Summary onSwitchPage={setPage} />;
};

export default ConnectedPage;
