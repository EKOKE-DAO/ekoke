const DECIMALS = 8;

export const convertToHumanReadable = (
  value: bigint,
  decimals: number = DECIMALS,
): string => {
  if (value === BigInt(0)) {
    return '0';
  }

  const divisor = BigInt(10 ** decimals);

  const wholePart = value / divisor;
  const fractionalPart = value % divisor;

  const fractionalString = fractionalPart.toString().padStart(decimals, '0');

  return `${wholePart.toString()}.${fractionalString}`;
};
