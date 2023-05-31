# Multihop IBC router
The multihop IBC router is contract that can be queried by other contracts for routes to get and format memo fields for IBC transfers.
The routes within the IBC router are decided upon by the admin of the contract. Adding, updating and removing router is done by the admin.

Depending on the expected users of the router, the admin should probably be a multisig or a dao contract to prevent abuse in formatting the memo field hops and sending the funds to a bad receiver.
Each intermediate receiver should be an some sort of account or wallet managed by the admin.

## Use on Osmosis
For almost all tokens on Osmosis, there is no hop between Osmosis and the host chain. The only exception as of writing this is pstake. The easiest way to start using the router is to embed it in a contract or run it as a side car, depending on whether you trust any current running routers 
