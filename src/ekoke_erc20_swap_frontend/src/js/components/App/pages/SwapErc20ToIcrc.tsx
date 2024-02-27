import * as React from 'react';
import * as Icon from 'react-feather';
import { useConnectedMetaMask } from 'metamask-react';
import { useConnectedIcWallet } from 'react-ic-wallet';
import { Principal } from '@dfinity/principal';

import Web3Client from '../../../web3/Web3Client';
import { ChainId } from '../../MetamaskConnect';
import Container from '../../reusable/Container';
import Heading from '../../reusable/Heading';
import Link from '../../reusable/Link';
import Button from '../../reusable/Button';
import { Page, PageProps } from '../ConnectedPage';
import Ethereum from '../../svg/Ethereum';
import InternetComputer from '../../svg/InternetComputer';
import EthereumWhite from '../../svg/EthereumWhite';
import Input from '../../reusable/Input';
import { e8sToEkoke, validatePrincipal } from '../../../utils';
import Alerts from '../../reusable/Alerts';
import Paragraph from '../../reusable/Paragraph';

const SwapErc20ToIcrc = ({ onSwitchPage }: PageProps) => {
  const { account, ethereum, chainId } = useConnectedMetaMask();
  const { principal } = useConnectedIcWallet();

  const [recipientPrincipal, setRecipientPrincipal] = React.useState<string>(
    principal.toString(),
  );
  const [amount, setAmount] = React.useState<string>('');
  const [userBalance, setUserBalance] = React.useState<bigint>();
  const [processing, setProcessing] = React.useState<boolean>(false);
  const [error, setError] = React.useState<string | null>(null);
  const [success, setSuccess] = React.useState<boolean>(false);

  const onRecipientPrincipalChange = (
    e: React.ChangeEvent<HTMLInputElement>,
  ) => {
    setRecipientPrincipal(e.target.value);
  };
  const onAmountChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setAmount(e.target.value);
  };

  const validateUserAmount = (
    amount: string | number | readonly string[] | undefined,
  ): boolean => {
    if (typeof amount !== 'string') return false;

    if (isNaN(parseInt(amount))) {
      return false;
    }

    if (userBalance === undefined) return true;

    const userAmount = BigInt(amount);
    return userAmount <= userBalance.valueOf();
  };

  const onSwap = () => {
    setProcessing(true);

    // check if the user has enough balance
    if (!validateUserAmount(amount)) {
      setProcessing(false);
      setError('Insufficient balance.');
      return;
    }
    if (!validatePrincipal(recipientPrincipal)) {
      setProcessing(false);
      setError('Invalid principal.');
      return;
    }

    const principal = Principal.fromText(recipientPrincipal);
    const numAmount = BigInt(amount);

    const web3 = new Web3Client(account, ethereum, chainId as ChainId);
    web3
      .swap(principal, numAmount.valueOf())
      .then(() => {
        setProcessing(false);
        setAmount('');
        setError(null);
        setSuccess(true);
        setRecipientPrincipal('');
      })
      .catch((e) => {
        setProcessing(false);
        setError(`Swap failed: ${e.message}`);
        setSuccess(false);
      });
  };

  React.useEffect(() => {
    if (!ethereum || !account || !chainId) return;

    const web3 = new Web3Client(account, ethereum, chainId as ChainId);

    web3.balanceOf(account).then((balance) => {
      setUserBalance(balance);
    });
  }, [ethereum, account, chainId]);

  const disabled = !recipientPrincipal || !amount || processing;

  return (
    <Container.FlexCols className="items-center justify-center">
      <Container.Card className="px-12 sm:px-4">
        <Container.FlexResponsiveRow className="items-center sm:items-start justify-between sm:justify-start gap-8">
          <Container.Container className="flex-0">
            {!processing && (
              <Link.IconLink
                className="hover:cursor-pointer"
                onClick={() => onSwitchPage(Page.Summary)}
              >
                <Icon.ArrowLeft className="mr-2 inline" />
                Back
              </Link.IconLink>
            )}
          </Container.Container>
          <Container.FlexRow className="flex-1 items-center justify-center gap-4">
            <Ethereum className="w-[32px] sm:hidden" />
            <Heading.H1 className="sm:text-lg">Swap ERC20 to ICRC</Heading.H1>
            <InternetComputer className="h-[32px] sm:hidden" />
          </Container.FlexRow>
        </Container.FlexResponsiveRow>
        {userBalance !== undefined && (
          <Container.Container className="py-4 text-text">
            <span>Your EKOKE ERC20 balance: {e8sToEkoke(userBalance)}</span>
          </Container.Container>
        )}
        <Container.FlexCols className="gap-4">
          <Container.Container>
            <Input.IconInput
              className="pl-[60px] sm:pl-[8px]"
              icon={
                <InternetComputer className="h-[20px] w-[40px] sm:hidden" />
              }
              label="Recipient Principal"
              id="recipient-principal"
              placeholder={principal.toString()}
              value={recipientPrincipal}
              validate={validatePrincipal}
              validationMessage="Please enter a valid principal."
              onChange={onRecipientPrincipalChange}
            />
          </Container.Container>
          <Container.Container>
            <Input.IconInput
              className="pl-[40px]"
              icon={<Icon.DollarSign size={20} className="inline" />}
              label="Amount"
              id="amount"
              value={amount}
              placeholder="10000"
              type="number"
              validationMessage="Please enter a valid amount."
              validate={validateUserAmount}
              onChange={onAmountChange}
            />
          </Container.Container>
          <Container.FlexCols className="items-center justify-center gap-8 sm:gap-2">
            {error && (
              <Alerts.Danger>
                <Paragraph.Default className="!text-left">
                  {error}
                </Paragraph.Default>
              </Alerts.Danger>
            )}
            {success && (
              <Alerts.Info>
                <span>
                  Swap successful! Wait up to 3 hours to see the swapped amount
                  on EKOKE ICRC .
                </span>
              </Alerts.Info>
            )}
            <Button.Cta onClick={onSwap} disabled={disabled}>
              {processing ? (
                <Icon.Loader className="inline mr-2 animate-spin" size={20} />
              ) : (
                <EthereumWhite className="inline mr-2 h-[20px]" />
              )}
              <span>Swap</span>
            </Button.Cta>
          </Container.FlexCols>
        </Container.FlexCols>
      </Container.Card>
    </Container.FlexCols>
  );
};

export default SwapErc20ToIcrc;
