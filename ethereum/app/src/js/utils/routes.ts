export enum Route {
  HOME = 'home',
  DEFERRED = 'deferred',
  EKOKE = 'ekoke',
  EKOKE_PRESALE = 'ekoke-presale',
  MARKETPLACE = 'marketplace',
  REWARD_POOL = 'reward-pool',
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Route {
  export function url(route: Route): string {
    if (route === Route.HOME) {
      return `/`;
    }

    return `/${route}`;
  }

  export function isRoute(keyOrValue: any): boolean {
    return isEnumKey(keyOrValue) || isEnumValue(keyOrValue);
  }

  export function marketplaceRealEstateUrl(id: number): string {
    return `/marketplace/real-estate/${id}`;
  }
}

function isEnumKey(key: any): boolean {
  return Object.keys(Route).includes(key);
}

function isEnumValue(value: any): boolean {
  return Object.values(Route).includes(value);
}

export interface MenuEntries {
  [key: string]: MenuEntry;
}

interface MenuEntry {
  label: string;
  link?: Route;
  section?: string;
}

export const getIdFromHash = (): string | undefined => {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const hash = window.location.hash;
  const lastIndex = hash.lastIndexOf('#');

  if (lastIndex < 0) {
    return undefined;
  }

  return hash.slice(lastIndex + 1);
};
