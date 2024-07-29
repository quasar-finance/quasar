# Quasar smart contracts

Verify sum:
```
shasum -a 256 smart-contracts/artifacts/lp_strategy.wasm 
```



## how to review prs

Changes to state.rs - if there are any map structure changes, make sure ALL of the changes are migrated over in migrate