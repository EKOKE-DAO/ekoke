import * as React from 'react';
import { ActorSubclass } from '@dfinity/agent';
import { useIcWallet } from 'react-ic-wallet';

import {
  EkokeLedger,
  idlFactory as ekokeLedgerIdlFactory,
} from './ekoke-ledger.did';
import {
  EkokeErc20Swap,
  idlFactory as ekokeErc20SwapIdlFactory,
} from './ekoke-erc20-swap.did';

const EKOKE_ERC20_SWAP_CANISTER_ID = '';
const EKOKE_LEDGER_CANISTER_ID = '';

interface Context {
  ekokeErc20Swap?: ActorSubclass<EkokeErc20Swap>;
  ekokeLedger?: ActorSubclass<EkokeLedger>;
}

export const AgentContext = React.createContext<Context>({});

const AgentContextProvider = ({ children }: { children?: React.ReactNode }) => {
  const [ekokeErc20Swap, setEkokeErc20Swap] =
    React.useState<ActorSubclass<EkokeErc20Swap>>();
  const [ekokeLedger, setEkokeLedger] =
    React.useState<ActorSubclass<EkokeLedger>>();

  const { createActor, status } = useIcWallet();

  React.useEffect(() => {
    if (status === 'connected') {
      createActor(EKOKE_ERC20_SWAP_CANISTER_ID, ekokeErc20SwapIdlFactory).then(
        (actor) => {
          setEkokeErc20Swap(actor as ActorSubclass<EkokeErc20Swap>);
        },
      );
      createActor(EKOKE_LEDGER_CANISTER_ID, ekokeLedgerIdlFactory).then(
        (actor) => {
          setEkokeLedger(actor as ActorSubclass<EkokeLedger>);
        },
      );
    } else {
      setEkokeErc20Swap(undefined);
      setEkokeLedger(undefined);
    }
  }, [status]);

  return (
    <AgentContext.Provider value={{ ekokeErc20Swap, ekokeLedger }}>
      {children}
    </AgentContext.Provider>
  );
};

export default AgentContextProvider;
