# ICQ

This is an *IBC Enabled* contract that allows us to send ICQ queries from one chain over the standard ICQ
protocol to the bank module of another chain.

## Workflow

The contract starts with minimal state. It just stores a default timeout in seconds for all packets it sends.
Most importantly it binds a local IBC port to enable channel connections.

An external party first needs to make one or more channels using this contract as one endpoint. It will use standard 
unordered channels for the version negotiation. Once established, it manages a list of known channels. 

After there is at least one channel, you can send any ICQ query to this contract. It may optionally include a custom timeout.

## Messages

It only accepts ICQQueryMsg. The data sent along with that message must be a JSON-serialized
TransferMsg:

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
