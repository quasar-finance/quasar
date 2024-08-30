# Set up - 
- Up the quasar chain
- Up the cosmos-hub chain
- Up the osmosis chain
- Relayer connect quasar and cosmos-hub
- Relayer connect quasar and osmosis
- Relayer connect osmosis and cosmos-hub

# Scenarios 
- IBC transfer 10000 uatom from cosmos-hub to quasar to alice account.
- IBC transfer 30000 uosmo from osmosis to quasar.
- Create pool uatom - usmo pool in osmosis.
- Deposit ibc uatom to orion vault using qbank for 7 days. And verify account balance. 
- Deposit ibc uosmo to orion vault using qbank for 7 days. And verify account balance.
- Verify the Join pool is happening or not. 
- Note down all the module accounts. 


# Prerequisites
1. `go` version 1.19https://github.com/osmohttps://github.com/osmosis-labs/osmosishttps://github.com/osmosis-labs/osmosissis-labs/osmosis
2. The cosmos-hub `gaia` repo should be cloned from  https://github.com/quasar-finance/gaia 
3. The osmosis `osmosis` repo should be cloned from https://github.com/osmosis-labs/osmosis 
4. The quasar `quasar` repo should be cloned from https://github.com/quasar-finance/quasar

## Steps 
- Create a demo directory in home directory. 
- Clone a quasar and create a following directory  structure. 
```bash
mkdir quasar-demo
cd quasar-demo
```
- clone quasar, gaia and osmosis.


## Up the quasar-chain, in the cloned quasar directory
```bash
cd quasar-demo/quasar/demos/orion-manual-demo/
./quasar_localnet.sh
```
You can do `tail -f quasar.log` to monitor quasar logs in a terminal.

## Up the osmosis chain, in the cloned osmosis (with ica) directory
```bash
cd quasar-demo/quasar/demos/orion-manual-demo/
./osmo_localnet.sh
```
You can do `tail -f osmosis.log` to monitor osmosis logs in a terminal.

## Up the cosmos-hub chain, in the gaid cloned directory 
```bash
cd quasar-demo/quasar/demos/orion-manual-demo/
./cosmos_localnet.sh
```
You can do `tail -f osmosis.log` to monitor osmosis logs in a terminal.

## Relayer setup

### copy hermes config 
```bash
cp ~/quasar-demo/quasar/demos/orion-manual-demo/hermes_config.toml ~/.hermes/config.toml
```

### hermes automatic setup

It is sufficient to just run the following:
```bash
./run_hermes.sh
```

### hermes manual setup ; Recommended for new devs who want to understand each steps
### Hermes Key restore 
```bash
hermes keys restore --mnemonic "jungle law popular reunion festival horn divorce quarter image gather october weird slide trend resource render abuse food tomorrow multiply price fun ask quarter" quasar
```

2022-06-01T06:24:07.459912Z  INFO ThreadId(01) using default configuration from '/home/ak/.hermes/config.toml'
Success: Restored key 'testkey1' (quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew) on chain quasar

```bash
hermes keys restore --mnemonic "blade trap agent boy note critic jazz nuclear eight lion pipe fresh tourist make broken inquiry close agree usual human stock move remain swim" cosmos
```
2022-06-01T06:24:15.776985Z  INFO ThreadId(01) using default configuration from '/home/ak/.hermes/config.toml'
Success: Restored key 'testkey2' (cosmos1lrelhs37akgz2wht0y377uerxjm9fh33ke3ksc) on chain cosmos

```bash
hermes keys restore --mnemonic "act scale exhibit enough swamp vivid bleak eagle giggle brass desert debris network scrub hazard fame salon normal over between inform advance sick dinner" osmosis
```

2022-06-01T06:24:30.371926Z  INFO ThreadId(01) using default configuration from '/home/ak/.hermes/config.toml'
Success: Restored key 'testkey3' (osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz) on chain osmosis

### Connecting the chains

### First pre-check relayer balances in each chain
```bash
quasard q bank balances quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew --node tcp://localhost:26659
gaiad q bank balances cosmos1lrelhs37akgz2wht0y377uerxjm9fh33ke3ksc  --node tcp://localhost:26669
osmosisd q bank balances osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz --node tcp://localhost:26679
```
### Connect quasar and cosmos 
```bash
hermes create connection quasar cosmos
```

- Expected Example Output -
- Connection handshake finished for [Connection {
    delay_period: 0ns,
    a_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "quasar",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-0",
            ),
        ),
    },
    b_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "cosmos",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-0",
            ),
        ),
    },
}]

Success: Connection {
    delay_period: 0ns,
    a_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "quasar",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-0",
            ),
        ),
    },
    b_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "cosmos",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-0",
            ),
        ),
    },
}


- Post connection - check the relayer balances again; You will observe gas fee deduction
- Post connection - check the connection using hermes command.

```bash
hermes query connections quasar
hermes query connections cosmos
hermes query connection end quasar connection-0
hermes query connection end cosmos connection-0
hermes query clients quasar 
hermes query clients cosmos
hermes query client state quasar 07-tendermint-0
hermes query client state cosmos 07-tendermint-0 
hermes query client state quasar 07-tendermint-1  
hermes query client connections  quasar 07-tendermint-0
hermes query client connections  quasar 07-tendermint-1
hermes query client connections  cosmos 07-tendermint-0
```


### Connect quasar and osmosis
```bash
hermes create connection quasar osmosis
```
- Expected output - 
  
Connection handshake finished for [Connection {
    delay_period: 0ns,
    a_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "quasar",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-1",
            ),
        ),
    },
    b_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "osmosis",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-0",
            ),
        ),
    },
}]

Success: Connection {
    delay_period: 0ns,
    a_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "quasar",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-1",
            ),
        ),
    },
    b_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "osmosis",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-0",
            ),
        ),
    },
}

- Post connection - check the relayer balances again; You will observe gas fee deduction
- Post connection - check the connection using hermes command.
```bash
hermes query connections quasar
hermes query connections cosmos
hermes query connection end quasar connection-0
hermes query connection end cosmos connection-0
hermes query clients quasar 
hermes query clients cosmos
hermes query clients osmosis
hermes query client state quasar 07-tendermint-0
hermes query client state quasar 07-tendermint-1 
hermes query client state cosmos 07-tendermint-0 
hermes query client state osmosis 07-tendermint-0
hermes query client connections  quasar 07-tendermint-0
hermes query client connections  quasar 07-tendermint-1
hermes query client connections  cosmos 07-tendermint-0
```

### Connect osmosis and cosmos hub
```bash
hermes create connection osmosis cosmos
```
- Expected output -

Connection handshake finished for [Connection {
    delay_period: 0ns,
    a_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "osmosis",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-1",
            ),
        ),
    },
    b_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "cosmos",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-1",
            ),
        ),
    },
}]

Success: Connection {
    delay_period: 0ns,
    a_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "osmosis",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-1",
            ),
        ),
    },
    b_side: ConnectionSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "cosmos",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: Some(
            ConnectionId(
                "connection-1",
            ),
        ),
    },
}

- Post connection - check the relayer balances aganin; You will observe gas fee deduction
- Post connection - check the connection using hermes command.

```bash
hermes query connections quasar
hermes query connections cosmos
hermes query connections osmosis
hermes query connection end quasar connection-0
hermes query connection end quasar connection-1
hermes query connection end cosmos connection-0
hermes query connection end cosmos connection-1
hermes query connection end osmosis connection-0
hermes query connection end osmosis connection-1

hermes query clients quasar 
hermes query clients cosmos
hermes query clients osmosis
hermes query client state quasar 07-tendermint-0
hermes query client state quasar 07-tendermint-1 
hermes query client state cosmos 07-tendermint-0 
hermes query client state cosmos 07-tendermint-1
hermes query client state osmosis 07-tendermint-0
hermes query client state osmosis 07-tendermint-1
hermes query client connections  quasar 07-tendermint-0
hermes query client connections  quasar 07-tendermint-1
hermes query client connections  cosmos 07-tendermint-0
hermes query client connections  cosmos 07-tendermint-1
hermes query client connections  osmosis 07-tendermint-0
hermes query client connections  osmosis 07-tendermint-1
```


### IBC token transfer channel creation 

### Create token transfer channel between cosmos and quasar 
```
hermes create channel --port-a transfer --port-b transfer cosmos connection-0
```
- Expected output - 
Success: Channel {
    ordering: Unordered,
    a_side: ChannelSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "cosmos",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: ConnectionId(
            "connection-0",
        ),
        port_id: PortId(
            "transfer",
        ),
        channel_id: Some(
            ChannelId(
                "channel-0",
            ),
        ),
        version: None,
    },
    b_side: ChannelSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "quasar",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: ConnectionId(
            "connection-0",
        ),
        port_id: PortId(
            "transfer",
        ),
        channel_id: Some(
            ChannelId(
                "channel-0",
            ),
        ),
        version: None,
    },
    connection_delay: 0ns,
}

- Hermes query channel status post checks  
```
hermes query channels quasar
hermes query channels cosmos
hermes query channels osmosis
```
### Create token transfer channel between cosmos and osmosis
```
hermes create channel --port-a transfer --port-b transfer cosmos connection-1
```

- Expected output - 
Success: Channel {
    ordering: Unordered,
    a_side: ChannelSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "cosmos",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: ConnectionId(
            "connection-1",
        ),
        port_id: PortId(
            "transfer",
        ),
        channel_id: Some(
            ChannelId(
                "channel-1",
            ),
        ),
        version: None,
    },
    b_side: ChannelSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "osmosis",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: ConnectionId(
            "connection-1",
        ),
        port_id: PortId(
            "transfer",
        ),
        channel_id: Some(
            ChannelId(
                "channel-0",
            ),
        ),
        version: None,
    },
    connection_delay: 0ns,
}

- Hermes query channel status post checks  
```
hermes query channels quasar
hermes query channels cosmos
hermes query channels osmosis
```

### Create token transfer channel between osmosis and quasar
```
hermes create channel --port-a transfer --port-b transfer quasar connection-1
```

Success: Channel {
    ordering: Unordered,
    a_side: ChannelSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "quasar",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-1",
        ),
        connection_id: ConnectionId(
            "connection-1",
        ),
        port_id: PortId(
            "transfer",
        ),
        channel_id: Some(
            ChannelId(
                "channel-1",
            ),
        ),
        version: None,
    },
    b_side: ChannelSide {
        chain: BaseChainHandle {
            chain_id: ChainId {
                id: "osmosis",
                version: 0,
            },
            runtime_sender: Sender { .. },
        },
        client_id: ClientId(
            "07-tendermint-0",
        ),
        connection_id: ConnectionId(
            "connection-0",
        ),
        port_id: PortId(
            "transfer",
        ),
        channel_id: Some(
            ChannelId(
                "channel-1",
            ),
        ),
        version: None,
    },
    connection_delay: 0ns,
}

- Hermes query channel status post checks  
```
hermes query channels quasar
hermes query channels cosmos
hermes query channels osmosis
```
### Detailed channel status commands 

With queries you should be able to track the associated self connection-id, self client-id, counterparty chain-id, counterparty client id, and counterparty connection-id

Tracking Hint 
- Channel ID - > [ Self - Connection ID, Counterparty port id, Counterparty channel id ]-> [ Client -ID, Counterparty clientid, Counterparty connection -id ] -> counterparty party Chain-ID


```bash
hermes query channel end quasar transfer channel-0
hermes query channel end quasar transfer channel-1

hermes query channel end cosmos transfer channel-0
hermes query channel end cosmos transfer channel-1

hermes query channel end osmosis transfer channel-0
hermes query channel end osmosis transfer channel-1
```
### Start hermes

```bash
hermes start
```

## IBC token transfer 
- Prechecks all account lists 
```bash
  gaiad keys list --home ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/cosmos-hub/
  quasard keys list --home ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/quasarnode/
  osmosisd keys list --home ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/osmosis/
```

- Prechecks account balances

```bash
gaiad q bank balances cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu  --node tcp://localhost:26669
gaiad q bank balances cosmos1twes4wv4c28r0x6dnczgda5sm36khlv7ve8m89  --node tcp://localhost:26669

quasard q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
quasard q bank balances quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu  --node tcp://localhost:26659

osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d --node tcp://localhost:26679

```
### IBC token transfer from cosmos to quasar
- Precheck account balances 

```
gaiad q bank balances cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu  --node tcp://localhost:26669
quasard q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
```

- IBC transfer
  
```
gaiad tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 1000uatom --from alice --chain-id cosmos --home ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/cosmos-hub  --node tcp://localhost:26669

```
- Post check account balances 
```
gaiad q bank balances cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu  --node tcp://localhost:26669
quasard q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
```

### IBC token transfer from osmosis to quasar
- Precheck balances 
```
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
quasard q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
```

- IBC Transfer 
  
```
osmosisd tx ibc-transfer transfer transfer channel-1 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100uosmo --from alice --chain-id osmosis --home ~/quasar-demo/quasar/demos/orion-manual-demo/run/home/osmosis/  --node tcp://localhost:26679
``` 

- Post check account balances 

```
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node tcp://localhost:26679
quasard q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec  --node tcp://localhost:26659
```
 