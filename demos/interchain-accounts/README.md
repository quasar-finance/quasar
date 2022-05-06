# ICA demo

## Setup

```bash
cd demos/interchain-accounts
```

```bash
./demo start_all
```

```bash
./demo init_relayer
```

## Register interchain account

```bash
./demo intertx_1_tx intertx register --connection-id connection-0
```

Now we can query the state of the registered account.

```bash
./demo intertx_1_q intertx interchainaccounts connection-0 cosmos1sqlsc5024sszglyh7pswk5hfpc5xtl77xrgn5a
```

It should return the account address created on host chain side:

```yaml
interchain_account_address: cosmos1prhhlqx4hsma6kl3wcvzkqhugs58s9jcpq985jpa6cr2ahewgxps2u0uds
```

## Fund ICA account

Check the balance of the ICA address on the host chain, it should be empty.

```bash
./demo intertx_2_q bank balances cosmos1prhhlqx4hsma6kl3wcvzkqhugs58s9jcpq985jpa6cr2ahewgxps2u0uds
```

Send some stake from Alice

```bash
./demo intertx_2_tx bank send alice cosmos1prhhlqx4hsma6kl3wcvzkqhugs58s9jcpq985jpa6cr2ahewgxps2u0uds 10000stake
```

Check the balance of the ICA address again, it should have the 10000stake.

```bash
./demo intertx_2_q bank balances cosmos1prhhlqx4hsma6kl3wcvzkqhugs58s9jcpq985jpa6cr2ahewgxps2u0uds
```

## Send a tx to host chain

Check bob's balance on host chain:

```bash
./demo intertx_2_q bank balances cosmos1ez43ye5qn3q2zwh8uvswppvducwnkq6w6mthgl
```

Now we tell the controller chain to instruct a bank transfer via the ICA account on host chain:

```bash
./demo intertx_1_tx intertx submit transfer.raw.json --connection-id connection-0
```
