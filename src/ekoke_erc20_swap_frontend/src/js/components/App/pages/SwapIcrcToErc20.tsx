import * as React from 'react';
import * as Icon from 'react-feather';
import { useConnectedMetaMask } from 'metamask-react';
import { useConnectedIcWallet } from 'react-ic-wallet';

import { Page, PageProps } from '../ConnectedPage';
import { e8sToEkoke, validateEthAddress } from '../../../utils';
import Container from '../../reusable/Container';
import Link from '../../reusable/Link';
import Ethereum from '../../svg/Ethereum';
import Heading from '../../reusable/Heading';
import InternetComputer from '../../svg/InternetComputer';
import Input from '../../reusable/Input';
import Alerts from '../../reusable/Alerts';
import Paragraph from '../../reusable/Paragraph';
import Button from '../../reusable/Button';

const SwapIcrcToErc20 = ({ onSwitchPage }: PageProps) => {
  const { account } = useConnectedMetaMask();
  const { principal } = useConnectedIcWallet();

  const [recipientAddress, setRecipientAddress] =
    React.useState<string>(account);
  const [amount, setAmount] = React.useState<string>('');
  const [userBalance, setUserBalance] = React.useState<BigInt>();
  const [processing, setProcessing] = React.useState<boolean>(false);
  const [error, setError] = React.useState<string | null>(null);
  const [swapTxHash, setSwapTxHash] = React.useState<string | false>(false);

  const onRecipientAddressChanged = (
    e: React.ChangeEvent<HTMLInputElement>,
  ) => {
    setRecipientAddress(e.target.value);
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
    if (!validateEthAddress(recipientAddress)) {
      setProcessing(false);
      setError('Invalid address.');
      return;
    }

    const numAmount = BigInt(amount);
  };

  const disabled = !recipientAddress || !amount || processing;

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
            <InternetComputer className="h-[32px] sm:hidden" />
            <Heading.H1 className="sm:text-lg">Swap ICRC to ERC20</Heading.H1>
            <Ethereum className="w-[32px] sm:hidden" />
          </Container.FlexRow>
        </Container.FlexResponsiveRow>
        {userBalance && (
          <Container.Container className="py-4 text-text">
            <span>Your EKOKE ICRC balance: {e8sToEkoke(userBalance)}</span>
          </Container.Container>
        )}
        <Container.FlexCols className="gap-4">
          <Container.Container>
            <Input.IconInput
              className="pl-[40px] sm:pl-[8px]"
              icon={<Ethereum className="h-[20px] sm:hidden" />}
              label="Recipient Ethereum Address"
              id="recipient-eth-address"
              placeholder={account}
              value={recipientAddress}
              validate={validateEthAddress}
              validationMessage="Please enter a valid ethereum address."
              onChange={onRecipientAddressChanged}
            />
          </Container.Container>
          <Container.Container>
            <Input.IconInput
              className="pl-[40px]"
              icon={<Icon.DollarSign size={20} className="inline" />}
              label="Amount"
              id="swap-amount"
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
            {swapTxHash && (
              <Alerts.Info>
                <Paragraph.Default className="!text-left">
                  Swap successful! See your transaction{' '}
                  <Link.Paragraph
                    href={`https://etherscan.io/tx/${swapTxHash}`}
                    target="_blank"
                  >
                    {swapTxHash}
                  </Link.Paragraph>
                </Paragraph.Default>
              </Alerts.Info>
            )}
            <Button.Cta onClick={onSwap} disabled={disabled}>
              {processing ? (
                <Icon.Loader className="inline mr-2 animate-spin" size={20} />
              ) : (
                <InternetComputer className="inline mr-2 h-[20px]" />
              )}
              <span>Swap</span>
            </Button.Cta>
          </Container.FlexCols>
        </Container.FlexCols>
      </Container.Card>
    </Container.FlexCols>
  );
};

export default SwapIcrcToErc20;
