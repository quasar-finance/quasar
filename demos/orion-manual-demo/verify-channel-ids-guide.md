This purpose of this guide is to demonstrate how to find the channel IDs in your test setup and
find out if they match what we expect in the demos of this directory.

1. First we need to verify channel IDs among chains.
   All channels of a chain can be listed by:
```
hermes query channels osmosis
hermes query channels cosmos
hermes query channels quasar
```
Focus on channels whose port ID is "transfer".
For these channels view their details by:
```
hermes query channel ends osmosis transfer <channel-id>
```
For example:
```
hermes query channel ends osmosis transfer channel-0
```
Check their chain_id, counterparty_chain_id, channel_id, and counterparty_channel_id.
the channel IDs should be as follows:
quasar->osmosis channel-1
osmosis->quasar channel-1
quasar->cosmos  channel-0
cosmos->quasar  channel-0
cosmos->osmosis channel-1
osmosis->cosmos channel-0

2. If the channel IDs found in previous step are different,
    edit the `complete_zone_info_map-proposal.json` accordingly.
