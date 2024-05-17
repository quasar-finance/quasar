# Merkle Incentives

The merkle incentives contract uses a merkle tree generated from a list of addresses and corresponding token amounts for those addresses.

The bulk of the logic sits inside of src/incentives/helpers.rs. In the function is_valid_claim

The merkle tree is generated from a csv with an arbitrary number of columns, where the first column is the user address and the next columns are the token amounts for that user. The csv is then hashed to create the merkle root.

The only constraint on the token amounts is that they must be sorted alphabetically. so this would be valid:

```
osmo123,1uatom,1usmo
osmo124,4uatom,4usmo
```

but this would not:

```
osmo123,1usmo,1uatom
osmo124,4usmo,4uatom
```

The merkle proof/root generator will strip commas and spaces from the csv before hashing it. This is the same as the behavior inside of the contract. This is important to keep in mind when generating a merkle tree from a database rather than a csv.

during the claim, the contract will automatically sort the tokens and strip commas/whitespace, so no additional work is required on the client side in order to claim, as long as they have the proper proof generated.

## Updating merkle root
The merkle root of the contract is updated by the incentives admin. This admin is an address set at instantiation and can be updated by the contract admin. 

## Test

The test data directory contains a csv called testdata.csv. This is the source document that a merkle tree can be generated on. As you can see we can easily add more rows to add users and more columns to add more incentive tokens.

To test the proof verification mechanism, we make sure to build the whole smart-contracts directory so that the merkle-cli rust binary is built. Then we can run the following command to generate a merkle root, then a proof for a user that wants to claim:

(run the following from the root of the merkle-incentives contract)

```bash
../../target/debug/merkle-cli generate-root testdata/testdata.csv

# Expected output: rZh9kBgioPQRC3R6LzoFpYmMJ81IUY5nTVr+X5/OsXI=
```

Then we can generate a proof for a user:

```bash
# this is the first entry in the csv, we can take any entry of course
../../target/debug/merkle-cli generate-proof testdata/testdata.csv osmo10004ufcv2aln3vl8defyk9agv5kacrzpkyw5p47uosmo1uxyz testdata/proof_data.json

# the output of this will be saved in testdata/proof_data.json
```

Then we can verify a proof for a user:

```bash
# this is the first entry in the csv, we can take any entry of course
../../target/debug/merkle-cli verify-proof rZh9kBgioPQRC3R6LzoFpYmMJ81IUY5nTVr+X5/OsXI= osmo10004ufcv2aln3vl8defyk9agv5kacrzpkyw5p47uosmo1uxyz testdata/proof_data.json

# the output if everything was correct should be The proof is succesfully verified. Given data is present in the Merkle Tree
# We can change slightly either the root or the claim string and we will see that we don't get the proof verified
```

## Good to know

If you see the testdata.csv file, you will see that multiple users can have entries in the merkle root, allowing for multiple claims per user. I don't like this architecture, but if we want to do this, the contract will work for it out of the box.

## Future work

This contract should be tested extensively against the following cases:

- A user tries to claim with a proof that is not valid
- A user tries to claim with a proof that is valid, against a valid root, but has already claimed
- A user has already claimed, the root is updated, and they:
  - Try to claim again when they have additional incentives
  - Try to claim again when they have the same incentives
