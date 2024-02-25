const DECIMALS = 8;

export const e8sToEkoke = (e8s: BigInt): string => {
  // put comma in `decimals` position
  const supplyStr = e8s.toString();
  const arr = supplyStr.split('');
  arr.splice(arr.length - DECIMALS, 0, '.');

  return arr.join('');
};
