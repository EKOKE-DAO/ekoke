import { expect, test } from "vitest";
import { Principal } from "@dfinity/principal";

import { sellContract } from "./actor.js";

test("should be able to update fly canister principal", async () => {
  const principal = Principal.fromText("rrkah-fqaaa-aaaaa-aaaaq-cai");
  await sellContract.admin_set_fly_canister(principal);
});

test("should be able to update marketplace canister principal", async () => {
  const principal = Principal.fromText("rrkah-fqaaa-aaaaa-aaaaq-cai");
  await sellContract.admin_set_marketplace_canister(principal);
});
