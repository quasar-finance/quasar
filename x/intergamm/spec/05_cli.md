<!--
order: 5
-->

The Intergamm keeper functions are not meant to be used via CLI, however the CLI functions are available for testing purposes.

These are the message server functions callable by CLI.

```protobuf
service Msg {
  rpc RegisterAccount(MsgRegisterAccount) returns (MsgRegisterAccountResponse);
  rpc CreatePool(MsgCreatePool) returns (MsgCreatePoolResponse);
  rpc JoinPool(MsgJoinPool) returns (MsgJoinPoolResponse);
  rpc ExitPool(MsgExitPool) returns (MsgExitPoolResponse);
  rpc IbcTransfer(MsgIbcTransfer) returns (MsgIbcTransferResponse);
}
```
