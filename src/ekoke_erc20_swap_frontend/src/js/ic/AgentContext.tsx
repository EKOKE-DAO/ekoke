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
import {
  CkEthLedger,
  idlFactory as ckEthLedgerIdlFactory,
} from './cketh-ledger.did';
import { Principal } from '@dfinity/principal';

const CKETH_LEDGER_CANISTER_ID = 'ss2fx-dyaaa-aaaar-qacoq-cai';
const EKOKE_ERC20_SWAP_CANISTER_ID = 'ss2fx-dyaaa-aaaar-qacoq-cai'; // FIXME: change
const EKOKE_LEDGER_CANISTER_ID = 'ss2fx-dyaaa-aaaar-qacoq-cai'; // FIXME: change

export const ekokeErc20SwapAccount: {
  owner: Principal;
  subaccount: [] | [Uint8Array | number[]];
} = {
  owner: Principal.fromText(EKOKE_ERC20_SWAP_CANISTER_ID),
  subaccount: [],
};

interface Context {
  ckEthLedger?: ActorSubclass<CkEthLedger>;
  ekokeErc20Swap?: ActorSubclass<EkokeErc20Swap>;
  ekokeLedger?: ActorSubclass<EkokeLedger>;
}

const AgentContext = React.createContext<Context>({});

const AgentContextProvider = ({ children }: { children?: React.ReactNode }) => {
  const [ckEthLedger, setCkEthLedger] =
    React.useState<ActorSubclass<CkEthLedger>>();
  const [ekokeErc20Swap, setEkokeErc20Swap] =
    React.useState<ActorSubclass<EkokeErc20Swap>>();
  const [ekokeLedger, setEkokeLedger] =
    React.useState<ActorSubclass<EkokeLedger>>();

  const { createActor, status } = useIcWallet();

  React.useEffect(() => {
    if (status === 'connected') {
      createActor(CKETH_LEDGER_CANISTER_ID, ckEthLedgerIdlFactory).then(
        (actor) => {
          setCkEthLedger(actor as ActorSubclass<CkEthLedger>);
        },
      );
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
    <AgentContext.Provider value={{ ckEthLedger, ekokeErc20Swap, ekokeLedger }}>
      {children}
    </AgentContext.Provider>
  );
};

export default AgentContextProvider;

export const useAgentContext = () => React.useContext(AgentContext);
