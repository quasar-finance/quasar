# IBC Packet forwarder demo

This demo tutorial demonstrates the working of multi-hop packet forwarding by using three chains from their latest source code in the local environment. The three chains used for the demo purpose are `quasar`, `cosmos-hub`, and `osmosis`.

Repo links
1. [quasar](https://github.com/quasar-finance/quasar)
2. [cosmos-hub](https://github.com/quasar-finance/gaia)
3. [osmosis](https://github.com/osmosis-labs/osmosis)
4. [multi-hop packet forwarder](https://github.com/strangelove-ventures/packet-forward-middleware)

These 3 blockchains will be communicating for the purpose of demonstrating a token transfer from `cosmos` to `quasar`, then from `quasar` to osmosis, using the IBC packet forwarding feature.

## Prerequisites

1. Both the `gaia` and `osmosis` repositories need to be cloned in a `contrib/` directory at the same level as the `quasar` repository.
2. The cosmos-hub `gaia` repo should be cloned from our fork https://github.com/quasar-finance/gaia and the branch `bugfix/replace_default_transfer_with_router_module` should be checked out.
3. `osmosis` require go version 1.18
4. `ignite` latest version should be installed (see https://docs.ignite.com/guide/install.html)
5. `gnome terminal` should also be installed if not already installed, to spawn terminal windows.

For ubuntu:

```bash
sudo apt-get install gnome-terminal
```

## Setup

1. Go to the `quasar` cloned directory, and cd `demos/packet-forwarder`. All steps below will be run from this directory.

```bash
cd demos/packet-forwarder
```

A `demo` script is there to run all the demo steps.

2. Start the 3 blockchains locally

```bash
./demo start_all
```

3. Configure and start the `transfer` channel on ignite relayer only and wait for it to finish creating the connections, it might take a couple of minutes.

```bash
./demo init_relayer
```

Now the 3 blockchains are able to communicate.

## Token transfer scenario

1. Get the Alice's address in the quasar chain.
Users' addresses are already fixed in the yaml config files file with mnemonics.

```bash
quasarnoded keys list --home run/quasar/home/
```

Alice's address on quasar is: `quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec`

2. Check that Alice on Quasar does not have yet any ATOM:

```bash
curl http://localhost:1311/bank/balances/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
```

or

```bash
quasarnoded q bank balances $(quasarnoded keys show -a alice --home run/quasar/home) --home run/quasar/home/ --node http://localhost:26659
```

3. Now Bob transfers 2000 uatom from `cosmos` to `quasar`

```bash
./demo tx_bob_cosmos_to_alice_quasar
```

Now the new ATOM transferred to alice on `quasar` should be visible:

```bash
curl http://localhost:1311/bank/balances/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
```

or

```bash
quasarnoded q bank balances $(quasarnoded keys show -a alice --home run/quasar/home) --home run/quasar/home/ --node http://localhost:26659
```

3. Alice has the ATOM available in the form of an IBC token on `quasar`. We now transfer it to `osmosis` but doing a multi-hop transaction via `cosmos` using the packet forwarder.

Alice's address on osmosis is: `osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq`

Therefore the receiver address looks like:
`cosmos1vzxkv3lxccnttr9rs0002s93sgw72h7ghukuhs|transfer/channel-1:osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq`

The initial cosmos address can be a random address. It is in fact a temporary address that will hold the denom + amount, from which the fee will be deducted and retained by cosmos, before being forwarded to osmosis.

We check first that the receiver on `osmosis` does not yet have the atom balance.

```bash
curl http://localhost:1312/bank/balances/osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq
```

or

```bash
osmosisd q bank balances $(osmosisd keys show -a alice --home run/osmosis/home) --home run/osmosis/home/ --node http://localhost:26559
```

Then we make the tx:

```bash
./demo tx_alice_quasar_to_alice_osmosis_via_cosmos
```

And we check the balance again for Alice on `osmosis`:
```bash
curl http://localhost:1312/bank/balances/osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq
```

or

```bash
osmosisd q bank balances $(osmosisd keys show -a alice --home run/osmosis/home) --home run/osmosis/home/ --node http://localhost:26559
```

It should display the 1000 IBC denom for the original ATOM.

4. Send 1000 uatom (one hop ibc transfer) from cosmos-hub to osmosis using alice as sender and receiver.

```bash
/demo tx_alice_cosmos_to_alice_osmosis
```

and then verify the balance:

```bash
curl http://localhost:1312/bank/balances/osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq
```

or

```bash
osmosisd q bank balances $(osmosisd keys show -a alice --home run/osmosis/home) --home run/osmosis/home/ --node http://localhost:26559
```

This step is to verify that the final denom that reaches to osmosis via both the paths are same.

1. Path #1 `quasar` (one hop atom hex hash ( `cosmos-hub` -> `quasar` ) -> `cosmos-hub` -> `osmosis`
2. Path #2 `cosmos-hub` -> `osmosis`

This step should update the alice ibc hex hash atom balance to be increased by the sent amount.
