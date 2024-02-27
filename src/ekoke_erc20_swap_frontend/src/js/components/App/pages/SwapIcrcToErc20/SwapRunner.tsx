import * as React from 'react';
import { CanisterSwapState } from '../SwapIcrcToErc20';
import { Principal } from '@dfinity/principal';

import {
  ekokeErc20SwapAccount,
  useAgentContext,
} from '../../../../ic/AgentContext';
import TaskList from '../../../TaskList/TaskList';

export interface RunState {
  account: {
    owner: Principal;
    subaccount: [] | [Uint8Array | number[]];
  };
  canisterSwapState: CanisterSwapState;
  recipientAddress: string;
  swapAmount: bigint;
}

interface Props {
  run: RunState | null;
  onDone: () => void;
  onTxHash: (txHash: string) => void;
}

const SwapRunner = ({ run, onDone, onTxHash }: Props) => {
  const { ckEthLedger, ekokeLedger, ekokeErc20Swap } = useAgentContext();
  const { canisterSwapState, recipientAddress, account, swapAmount } =
    run || {};

  // states
  const [gasFee, setGasFee] = React.useState<bigint>();
  const [ckEthAllowance, setCkEthAllowance] = React.useState<bigint>();
  const [ekokeAllowance, setEkokeAllowance] = React.useState<bigint>();
  const [txHash, setTxHash] = React.useState<string | null>(null);

  const getGasFee = async () => {
    if (ekokeErc20Swap === undefined)
      return { error: 'ekoke Erc20 swap canister unavailable' };
    if (canisterSwapState === undefined)
      return { error: 'canister swap state unavailable' };
    const feeResult = await ekokeErc20Swap.swap_fee();
    if ('Ok' in feeResult) {
      setGasFee(feeResult.Ok + canisterSwapState.ckEthIcrcFee);
      return true;
    }
    return { error: 'failed to get swap fee' };
  };

  const getCkEthAllowance = async () => {
    if (account === undefined) return { error: 'account unavailable' };
    if (ckEthLedger === undefined) return { error: 'ckEth ledger unavailable' };
    const allowanceResult = await ckEthLedger.icrc2_allowance({
      spender: ekokeErc20SwapAccount,
      account,
    });
    setCkEthAllowance(allowanceResult.allowance);

    return true;
  };

  const giveCkEthAllowance = async () => {
    if (account === undefined) return { error: 'account unavailable' };
    if (ckEthLedger === undefined) return { error: 'ckEth ledger unavailable' };
    if (ekokeLedger === undefined) return { error: 'ekoke ledger unavailable' };
    if (ckEthAllowance === undefined)
      return { error: 'ckEth allowance unavailable' };
    if (gasFee === undefined) return { error: 'gas fee unavailable' };
    // check whether allowance is enough
    if (ckEthAllowance >= gasFee) return true;

    // otherwise give allowance
    const res = await ekokeLedger.icrc2_approve({
      fee: [],
      memo: [],
      from_subaccount: [],
      created_at_time: [],
      amount: gasFee,
      expected_allowance: [],
      expires_at: [],
      spender: ekokeErc20SwapAccount,
    });

    if ('Ok' in res) {
      return true;
    }

    return { error: 'failed to give allowance to the ckEth canister' };
  };

  const getEkokeAllowance = async () => {
    if (account === undefined) return { error: 'account unavailable' };
    if (ekokeLedger === undefined) return { error: 'ekoke ledger unavailable' };
    const allowanceResult = await ekokeLedger.icrc2_allowance({
      spender: ekokeErc20SwapAccount,
      account,
    });
    setEkokeAllowance(allowanceResult.allowance);

    return true;
  };

  const giveEkokeAllowance = async () => {
    if (account === undefined) return { error: 'account unavailable' };
    if (ekokeLedger === undefined) return { error: 'ekoke ledger unavailable' };
    if (ekokeAllowance === undefined)
      return { error: 'ekoke allowance unavailable' };
    if (swapAmount === undefined) return { error: 'amount unavailable' };
    if (canisterSwapState === undefined)
      return { error: 'canister swap state unavailable' };
    // check whether allowance is enough
    const totalAllowanceAmount = swapAmount + canisterSwapState.ekokeIcrcFee;
    if (ekokeAllowance >= totalAllowanceAmount) return true;

    // otherwise give allowance
    const res = await ekokeLedger.icrc2_approve({
      fee: [],
      memo: [],
      from_subaccount: [],
      created_at_time: [],
      amount: totalAllowanceAmount,
      expected_allowance: [],
      expires_at: [],
      spender: ekokeErc20SwapAccount,
    });

    if ('Ok' in res) {
      return true;
    }

    return { error: 'failed to give allowance to the ekoke canister' };
  };

  const swap = async () => {
    if (swapAmount === undefined) return { error: 'amount unavailable' };
    if (ekokeErc20Swap === undefined)
      return { error: 'ekoke ERC20 swap canister unavailable' };
    if (recipientAddress === undefined)
      return { error: 'recipient address unavailable' };

    const res = await ekokeErc20Swap.swap(recipientAddress, swapAmount, []);

    if ('Ok' in res) {
      setTxHash(res.Ok);
      return true;
    }

    return { error: 'failed to swap' };
  };

  React.useEffect(() => {
    if (txHash !== null) {
      onTxHash(txHash);
    }
  }, [txHash]);

  return (
    <TaskList
      run={run !== undefined}
      onDone={onDone}
      title="Swap ICRC to ERC20"
      tasks={[
        {
          label: 'Get Swap Gas Fee',
          action: getGasFee,
        },
        {
          label: 'Get CKETH Allowance',
          action: getCkEthAllowance,
        },
        {
          label: 'Give CKETH Allowance',
          action: giveCkEthAllowance,
        },
        {
          label: 'Get EKOKE Allowance',
          action: getEkokeAllowance,
        },
        {
          label: 'Give EKOKE Allowance',
          action: giveEkokeAllowance,
        },
        {
          label: 'Swap Tokens to ERC20',
          action: swap,
        },
      ]}
    />
  );
};

export default SwapRunner;
