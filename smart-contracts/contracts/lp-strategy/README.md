# Quasar Strategy
A Quasar strategy is made to work in conjunction with the Quasar vault contract. Theoratically, the strategy contract can be deployed independent of the vault, however it's functionality will be limited.

## What does a Strategy do?
A strategy accepts funds, places them in some location where yield is earned and keeps tracks of the location of the funds.

## What does a Strategy not do?
A strategy does not do any bookkeeping or accounting on deposits from specific users, It also does not concern itself with the division of profit.
For this functionality, the vault contract should be used.