import * as React from 'react';

import Container from '../reusable/Container';
import Button from '../reusable/Button';

const DisconnectPopup = ({
  onDisconnect,
  onDismiss,
}: {
  onDisconnect: () => void;
  onDismiss: () => void;
}) => {
  return (
    <Container.Container className="h-screen left-0 overflow-hidden fixed right-0 top-0 w-screen z-50">
      <Container.Container className="bg-gray-800/60 h-screen w-screen" />
      <Container.Container className="bg-white bottom-0 h-fit sm:h-[70vh] sm:rounded-t-xl left-0 m-auto p-8 fixed right-0 top-0 sm:top-auto sm:bottom-0 w-fit min-w-[25%] sm:w-full">
        <Container.FlexCols className="gap-4 text-lg p-4">
          <span className="text-lg text-center block text-text">
            Do you want to disconnect your wallet?
          </span>
          <Container.FlexResponsiveRow className="items-center justify-between gap-8">
            <Button.Danger onClick={onDisconnect}>Disconnect</Button.Danger>
            <Button.Alternative onClick={onDismiss}>Cancel</Button.Alternative>
          </Container.FlexResponsiveRow>
        </Container.FlexCols>
      </Container.Container>
    </Container.Container>
  );
};

export default DisconnectPopup;
