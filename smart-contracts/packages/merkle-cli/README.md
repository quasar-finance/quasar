# Merkle CLI

## Addresses and Coins

### Generate Root
```bash
merkle-cli generate-root testdata/uosmo_only.csv
```

### Generate Proof
```bash
merkle-cli generate-proof testdata/uosmo_only.csv osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo --print
```

or

```bash
merkle-cli generate-proof testdata/uosmo_only.csv osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo testdata/proof_data.json
```

### Verify Proof
```bash
merkle-cli verify-proof Nz54SQtyBVHwsmEqNI//mxFgiq8MRD7sS92IGkhgMvo= osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo testdata/proof_data.json
```

## Addresses and Amounts

### Generate Root

```bash
merkle-cli generate-root testdata/address_amount.csv
```

Expected result:
```
1V0YcwzXWtB+iuOTob6juiNliUmB278xZIKMnzwjqOU=
```

### Generate Proof

```bash
merkle-cli generate-proof testdata/address_amount.csv osmo1hqslwuc8ukaaaxfmahgnquyqx3w0tmrluwxmxj1421901 testdata/proof_data_address_amount.json
```

### Verify Proof

```bash
merkle-cli verify-proof 1V0YcwzXWtB+iuOTob6juiNliUmB278xZIKMnzwjqOU= osmo1hqslwuc8ukaaaxfmahgnquyqx3w0tmrluwxmxj1421901 testdata/proof_data_address_amount.json
```
