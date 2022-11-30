# ICQ

This is an *IBC Enabled* contract that allows us to use ICA to ica enabled chains. The 
counterparty ibc-go version needs to be either [3.4.x, 4.2.x, 5.1.x, 6.0.x]

## Workflow

The contract starts with minimal state. It just stores a default timeout in seconds for all packets it sends.
Most importantly it binds a local IBC port to enable channel connections.

An external party first needs to make one or more channels using this contract as one endpoint. It will use 
ordered channels for the version negotiation. Once established, it manages a list of known channels.
The version set in the channel opening needs to be the json of the ica metadata, eg:
```json
{
  "version":"ics27-1",
  "encoding":"proto3",
  "tx_type":"sdk_multi_msg",
  "controller_connection_id":"connection-0",
  "host_connection_id":"connection-0"
}
```

After there is at least one channel, you can send any ICA transaction to this contract.

## Messages

It only accepts a hardcoded osmosis join pool messages. This should be used as an example of how to send tx to the counterparty chain

## Queries

Queries only make sense relative to the established channels of this contract.

* `Port{}` - returns the port ID this contract has bound, so you can create channels. This info can be queried 
  via wasmd contract info query, but we expose another query here for convenience.
* `ListChannels{}` - returns a (currently unpaginated) list of all channels that have been created on this contract.
  Returns their local channelId along with some basic metadata, like the remote port/channel and the connection they
  run on top of.
* `Channel{id}` - returns more detailed information on one specific channel. In addition to the information available
  in the list view, it returns the current outstanding balance on that channel, as well as the total amount that
  has ever been sent on the channel.
