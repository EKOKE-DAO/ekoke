const DECIMALS = 9;

export const convertToHumanReadable = (
  value: bigint,
  decimals: number = DECIMALS,
  hideDecimals: boolean = false,
): string => {
  if (value === BigInt(0)) {
    return '0';
  }

  const divisor = BigInt(10 ** decimals);

  const wholePart = value / divisor;

  if (hideDecimals) {
    return wholePart.toString();
  }

  const fractionalPart = value % divisor;

  const fractionalString = fractionalPart.toString().padStart(decimals, '0');

  return `${wholePart.toString()}.${fractionalString}`;
};
