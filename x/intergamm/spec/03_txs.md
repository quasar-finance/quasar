<!--
order: 3
-->

# Keepers

## Keeper functions

The Intergamm keeper exposes MsgServer functions to directly talk to the remote chain.

```protobuf
service Msg {
  rpc RegisterAccount(MsgRegisterAccount) returns (MsgRegisterAccountResponse);
  rpc CreatePool(MsgCreatePool) returns (MsgCreatePoolResponse);
  rpc JoinPool(MsgJoinPool) returns (MsgJoinPoolResponse);
  rpc ExitPool(MsgExitPool) returns (MsgExitPoolResponse);
  rpc IbcTransfer(MsgIbcTransfer) returns (MsgIbcTransferResponse);
}
```
