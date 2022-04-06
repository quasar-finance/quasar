## Queries

qbank module supports the following queries.

## Following CLI queries are being supported by the qbank module.

## Query the current total deposit of a user

```bash
quasarnoded query qbank user-deposit [user-acc] [flags]
```

## Query the current total denom-wise deposit

```bash
quasarnoded query qbank user-denom-deposit [user-acc] [flags]
```

## Query the users epoch wise lockup period despoit

This query will return tokens deposited on a specific epoch day with the specified lockup period and denom.

```bash
quasarnoded query qbank user-denom-epoch-lockup-deposit [user-acc] [denom] [epoch-day] [lockup-type] [flags]
```

TODO - should change the sequence of arguments.

## Query the current total withdrawable amount

```bash
quasarnoded query qbank user-withdraw [user-acc] [flags]
```

## Query the current total denom-wise withdrawable amount

```bash
quasarnoded query qbank user-denom-withdraw [user-acc] [denom] [flags]
```

## Query the current reward amount available for claim

```bash
quasarnoded query qbank user-claim-rewards [user-acc] [flags]
```
