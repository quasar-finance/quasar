<!--
order: 4
-->

# Queries

Epochs module is providing below queries to check the module's state.

```protobuf
service Query {
  // Queries a list of InterchainAccountFromAddress items.
	rpc InterchainAccountFromAddress(QueryInterchainAccountFromAddressRequest) returns (QueryInterchainAccountFromAddressResponse) {
		option (google.api.http).get = "/abag/quasarnode/intergamm/interchain_account_from_address";
	}
}
```
