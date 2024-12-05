import { ChainId } from '../../components/MetamaskConnect';

export const ABI = [
  {
    inputs: [
      {
        internalType: 'address',
        name: '_owner',
        type: 'address',
      },
      {
        internalType: 'address',
        name: '_ekoke',
        type: 'address',
      },
      {
        internalType: 'address',
        name: '_deferred',
        type: 'address',
      },
    ],
    stateMutability: 'nonpayable',
    type: 'constructor',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: 'owner',
        type: 'address',
      },
    ],
    name: 'OwnableInvalidOwner',
    type: 'error',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: 'account',
        type: 'address',
      },
    ],
    name: 'OwnableUnauthorizedAccount',
    type: 'error',
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'address',
        name: 'previousOwner',
        type: 'address',
      },
      {
        indexed: true,
        internalType: 'address',
        name: 'newOwner',
        type: 'address',
      },
    ],
    name: 'OwnershipTransferred',
    type: 'event',
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: 'uint256',
        name: 'reward',
        type: 'uint256',
      },
      {
        indexed: false,
        internalType: 'uint256',
        name: 'tokens',
        type: 'uint256',
      },
    ],
    name: 'PoolReserved',
    type: 'event',
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: 'address',
        name: 'to',
        type: 'address',
      },
      {
        indexed: false,
        internalType: 'uint256',
        name: 'amount',
        type: 'uint256',
      },
    ],
    name: 'RewardSent',
    type: 'event',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: '_marketplace',
        type: 'address',
      },
    ],
    name: 'adminSetMarketplace',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'availableReward',
    outputs: [
      {
        internalType: 'uint256',
        name: 'available',
        type: 'uint256',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'owner',
    outputs: [
      {
        internalType: 'address',
        name: '',
        type: 'address',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'uint256',
        name: '_reward',
        type: 'uint256',
      },
      {
        internalType: 'uint256',
        name: '_tokens',
        type: 'uint256',
      },
    ],
    name: 'reservePool',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [],
    name: 'reservedAmount',
    outputs: [
      {
        internalType: 'uint256',
        name: '',
        type: 'uint256',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: '_to',
        type: 'address',
      },
      {
        internalType: 'uint256',
        name: '_amount',
        type: 'uint256',
      },
    ],
    name: 'sendReward',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: 'newOwner',
        type: 'address',
      },
    ],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
];

interface ContractAddress {
  [ChainId.Goerli]: string;
  [ChainId.Kovan]: string;
  [ChainId.Mainnet]: string;
  [ChainId.Rinkeby]: string;
  [ChainId.Ropsten]: string;
  [ChainId.Sepolia]: string;
  [ChainId.Hardhat]: string;
}

export const CONTRACT_ADDRESS: ContractAddress = {
  [ChainId.Goerli]: '',
  [ChainId.Kovan]: '',
  [ChainId.Mainnet]: '',
  [ChainId.Rinkeby]: '',
  [ChainId.Ropsten]: '',
  [ChainId.Sepolia]: '0x221f2B77e40d3F07eABaC928B1AC7382BE11a7Bc',
  [ChainId.Hardhat]: '',
};
