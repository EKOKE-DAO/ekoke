import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../../src/declarations/sell_contract/sell_contract.did.js";
import canisterIds from "../../.dfx/local/canister_ids.json";

const createActor = async (canisterId, options) => {
  const agent = new HttpAgent({ ...options?.agentOptions });
  await agent.fetchRootKey();

  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options?.actorOptions,
  });
};

export const sellContract = await createActor(canisterIds.sell_contract, {
  agentOptions: { host: "http://localhost:8000", fetch },
});
