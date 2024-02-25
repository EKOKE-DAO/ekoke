import { Principal } from '@dfinity/principal';

const DECIMALS = 8;

export const e8sToEkoke = (e8s: BigInt): string => {
  // put comma in `decimals` position
  const supplyStr = e8s.toString();
  const arr = supplyStr.split('');
  arr.splice(arr.length - DECIMALS, 0, '.');

  return arr.join('');
};

export const validatePrincipal = (
  principal: string | number | readonly string[] | undefined,
): boolean => {
  if (typeof principal !== 'string') return false;

  try {
    Principal.fromText(principal);
    return true;
  } catch (_) {
    return false;
  }
};

export const validateIsNumber = (
  amount: string | number | readonly string[] | undefined,
): boolean => {
  if (typeof amount !== 'string') return false;

  return !isNaN(parseInt(amount));
};

export const principalToBytes32 = (principal: Principal): string => {
  const principalToHex = principal.toHex();
  // right align the hex string
  return '0x' + principalToHex.padEnd(64, '0');
};
