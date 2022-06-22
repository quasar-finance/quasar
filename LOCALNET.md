
# Quasar Local Testnet (localnet)

To run a small network of Quasar nodes locally, first generate their respective configuration:

## Setup

### Cosmovisor

Cosmovisor should be installed and available to use, [follow these instructions](https://github.com/cosmos/cosmos-sdk/tree/main/cosmovisor#installation).

Note, the current method for installation of the latest version of cosmovisor is broken, but you can still build it from source:

```bash
git clone git@github.com:cosmos/cosmos-sdk
cd cosmos-sdk
git checkout cosmovisor/v1.1.0
make cosmovisor
cp cosmovisor/cosmovisor ~/go/bin/cosmovisor
echo "$(which cosmovisor)"
```

```bash
make build # make sure we have a quasar executable available first
scripts/localnet init_default_cluster
```

This will create a set of configuration files per node, under `run/localnet`.

By default, there is a total of 4 nodes: `node1`, `node2`, `node3`, `node4`

The network is now ready to be started.

Note that the chain state will be kept under the `run/localnet` node folders.
Running `scripts/localnet init_default_cluster` a second time will fail if existing state is present in `run/localnet`.

If a reset is desired, the entire content of `run/localnet` can be removed and `scripts/localnet init_default_cluster` can be run again.

## Start

```bash
scripts/localnet start_all
```

To see if the nodes are running:

```bash
scripts/localnet status_all
```

Once the network is running, you can see the logging per node, this way:

```bash
scripts/localnet node 0 log
```

Here, `0` is the node identifier, you can use, `1`, `2`, or `3` to see the logging of the other nodes.

## Commands

You can also issue query or tx commands against each node:

```bash
scripts/localnet node 0 cmd tx bank send main quasar1khfcjt5w0dfjgkcudlrnnun2rtq359ulrgv7gw 1000uqsar
```

This will send a bank transfer from `node0`'s `main` address, to another quasar address `quasar1khfcjt5w0dfjgkcudlrnnun2rtq359ulrgv7gw`.

You can check the balance after the transfer:

```bash
curl http://localhost:1300/bank/balances/quasar1khfcjt5w0dfjgkcudlrnnun2rtq359ulrgv7gw
```

Note that the API for `node0` is available at `localhost:1300`, while the API for `node1` is at `localhost:1301`, and so forth.

## Test a chain upgrade

You can test a dummy upgrade by doing the following steps.

Once the network is running, you can first introduce a Quasar upgrade, using the following script. It will add source code for a dummy upgrade under `app/upgrades/dummy`:

```bash
scripts/add_dummy_upgrade
```

Then make sure the upgrade is registered in the `app.go`:

```golang
// imports
// ...
dummy "github.com/abag/quasarnode/app/upgrades/dummy"

// var block declaration
// ...
Upgrades = []upgrades.Upgrade{dummy.Upgrade}
```

Then you can recompile the quasar node binary and install it in the local node folders:

```bash
make build
scripts/localnet install_upgrade_binaries dummy
```

You can confirm that the new binaries are installed correctly and ready to be used:

```bash
ls run/localnet/node*/home/cosmovisor/upgrades/dummy/bin
```

Now we can trigger the upgrade via a governance proposal and `cosmovisor` will swap and restart the new binary.

You can also run a terminal window tailing the logging of one of the nodes (`scripts/localnet node 0 log`), to witness the upgrade happening.

```bash
scripts/localnet register_upgrade 1 BLOCK_HEIGHT dummy
```

Here we instruct the first proposal (`1`), to run the `dummy` upgrade at height `BLOCK_HEIGHT`.

The script will instruct all 4 validator nodes to vote yes on this proposal, that will happen at `BLOCK_HEIGHT`, choose a block height about a minute in the future, ~100 blocks away from now for instance, this will allow for the voting period to end (20 seconds configured) and the proposal to pass successfully.

After the target block height has been reached, you should see the restart happening in the logs, as well as the dummy print statement from the upgrade itself (`Dummy Quasar migration`).

## Stop

Last but not least, to stop the network:

```bash
scripts/localnet stop_all
```
