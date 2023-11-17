import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../../src/declarations/deferred/deferred.did.js";
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

export const deferred = await createActor(canisterIds.deferred.local, {
  agentOptions: { host: "http://localhost:4943", fetch },
});

export const fly = await createActor(canisterIds.fly.local, {
  agentOptions: { host: "http://localhost:4943", fetch },
});
