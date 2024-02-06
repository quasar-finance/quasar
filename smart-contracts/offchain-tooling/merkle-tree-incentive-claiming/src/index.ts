import { MerkleTree } from "./merkle-ts/src/merkle/MerkleTree";

async function main() {
  const tree = new MerkleTree([
    { address: "useraddress1", amountasset1: 100, amountasset2: 200 },
    { address: "useraddress2", amountasset1: 100, amountasset2: 200 },
    {
      address: "useraddress3",
      amountasset1: 100,
      amountasset2: 200,
      amountasset3: 300,
    },
  ]);

  console.log("Root: ", tree);
}

main();
