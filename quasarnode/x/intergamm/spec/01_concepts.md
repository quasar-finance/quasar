# Concepts
`Intergamm` is a wrapper module around osmosis `gamm` modules which enables remote calling `gamm` apis via IBC protocol.

The module is separated into 2 submodules: `Host` and `Controller`

## Host Module
Implemented in osmosis and is responsible for accepting channel initiation from other chains and receiving command packets.

### Channel Address
In order to create, join and exit pool in `gamm` module we need an account so we can deposit/withdraw funds. In host module we create an account for every channel with the following format to act as the owner or LP provider of the pools.
```
channel_address = gammaddr.Module(types.ModuleName, []byte(fmt.Sprintf("%s/%s", packet.SourcePort, packet.SourceChannel)))
```
We can also transfer funds from `channel_address` to another chains through IBC with `Withdraw` command.

## Controller
This module should be implemented in any chain that wants to interact with `gamm` module through IBC protocol. Initializing a channel connection is controller's responsible so always make sure to set the controller chain as `source` in relayer's config.