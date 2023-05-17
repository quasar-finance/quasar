# Multihop IBC router
The multihop IBC router is contract that can be queried by other contracts for routes to get and format memo fields for IBC transfers.
The routes within the IBC router are decided upon by the admin of the contract. Adding, updating and removing router is done by the admin.

Depending on the expected users of the router, the admin should probably be a multisig or a dao contract to prevent abuse in formatting the memo field hops and sending the funds to a bad receiver.
