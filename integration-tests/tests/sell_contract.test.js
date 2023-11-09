import { expect, test } from "vitest";
import { Principal } from "@dfinity/principal";

import { sellContract } from "./actor";

/*
test("should be able to update fly canister principal", async () => {
  const principal = Principal.fromText("rrkah-fqaaa-aaaaa-aaaaq-cai");
  await sellContract.admin_set_fly_canister(principal);
});

test("should be able to update marketplace canister principal", async () => {
  const principal = Principal.fromText("rrkah-fqaaa-aaaaa-aaaaq-cai");
  await sellContract.admin_set_marketplace_canister(principal);
});
*/

test("should be able to get the total amount of nfts", async () => {
  const totalSupply = await sellContract.total_supply();
  expect(totalSupply).toBeGreaterThanOrEqual(0);
});
