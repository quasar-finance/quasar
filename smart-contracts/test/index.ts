import prompts from "prompts";
import {
  mayhem,
  extreme_test,
  seed_liquidity_from_alice,
  simple_test,
} from "./vault/src/driver";
import { try_icq } from "./vault/src/vault";

async function main() {
  let response = await prompts({
    type: "text",
    name: "vaultAddress",
    message: "Enter the vault address",
  });
  console.log("vault addr:", response.vaultAddress);

  //   setInterval(async () => {
  //     try {
  //       await Promise.all([
  //         try_icq({
  //           vaultAddress: response.vaultAddress,
  //           from: 'alice',
  //         }),
  //       ])
  //     } catch (e) {
  //       //probably sequence issues
  //     }
  //   }, 13000)

  // await seed_liquidity_from_alice(response.vaultAddress)

  await simple_test(response.vaultAddress);
  //   await extreme_test(response.vaultAddress)
  // await mayhem(response.vaultAddress)
}

main();
