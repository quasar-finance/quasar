# IBC Packet forwarder demo

We want to setup and run 3 blockchains locally, `quasar`, `cosmos`, and `osmosis`.
These 3 blockchains will be communicating for the purpose of demonstrating a token transfer from `cosmos` to `quasar`, then from `quasar` to osmosis, using the IBC packet forwarding feature.

A script is there to run all the demo steps.

## Setup

1. Start 3 blockchains locally

```bash
./demo start_all
```

1. Configure and start the `transfer` channel on ignite relayer only and wait for it to finish creating the connections, it might take a couple of minutes.

```bash
./demo init_relayer
```

Now the 3 blockchains are able to communicate.

## Token transfer scenario

1. Check that Alice on Quasar does not have yet any ATOM:

```bash
curl http://localhost:1311/bank/balances/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
```

1. Now Bob transfers 2000 uatom from `cosmos` to `quasar`

```bash
./demo tx_bob_cosmos_to_alice_quasar
```

Now the new ATOM transferred to alice on `quasar` should be visibile:

```bash
curl http://localhost:1311/bank/balances/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
```

1. Alice has the ATOM available in the form of an IBC token on `quasar`. We now transfer it to `osmosis` but doing a multi-hop transaction via `cosmos` using the packet forwarder.

The receiver address looks like:
`cosmos1vzxkv3lxccnttr9rs0002s93sgw72h7ghukuhs|transfer/channel-1:osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq`

We check first that the receiver on `osmosis` does not yet have the atom balance.

```bash
curl http://localhost:1312/bank/balances/osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq
```

Then we make the tx:

```bash
./demo tx_bob_cosmos_to_quasar
```

And we check the balance again for Alice on `osmosis`:
```bash
curl http://localhost:1312/bank/balances/osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq
```

It should display the 1000 IBC denom for the original ATOM.
