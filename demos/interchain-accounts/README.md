# ICA demo

This demo tutorial demonstrates how interchain accounts work. It is the official interchain accounts demo, with some of the steps scripted.

Repo links
1. [Interchain account demo](https://github.com/cosmos/interchain-accounts-demo)

## Prerequisites

1. Clone the ICA demo repository in a `contrib/` directory at the same level as the `quasar` repository.

```bash
cd ../contrib
git clone git@github.com:cosmos/interchain-accounts-demo.git
```

2. `ignite` latest version should be installed (see https://docs.ignite.com/guide/install.html)
3. `gnome terminal` should also be installed if not already installed, to spawn terminal windows.

For ubuntu:

```bash
sudo apt-get install gnome-terminal
```

## Setup

1. Go to the `quasar` repo directory, and cd `intergamm-osmosis`. All steps below will be run from this directory.

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
./demo intertx_1_tx intertx register --connection-id connection-0 -y
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
./demo intertx_2_tx bank send alice cosmos1prhhlqx4hsma6kl3wcvzkqhugs58s9jcpq985jpa6cr2ahewgxps2u0uds 10000stake -y
```

Check the balance of the ICA address again for the ICA account, it should have the 10000stake.


## Send a tx to host chain

Check bob's balance on host chain:

```bash
./demo intertx_2_q bank balances cosmos1ez43ye5qn3q2zwh8uvswppvducwnkq6w6mthgl
```

Now we tell the controller chain to instruct a bank transfer via the ICA account on host chain:

```bash
./demo intertx_1_tx intertx submit transfer.raw.json --connection-id connection-0 -y
```

Check the balance of the ICA address again for Bob, it should have the extra 42stake.

```bash
./demo intertx_2_q bank balances cosmos1ez43ye5qn3q2zwh8uvswppvducwnkq6w6mthgl
```
