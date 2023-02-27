# A few important code references to understand the low level code flow. 

## If you are a debugger - I recommend to use debugger and set breakpoints in below methods 

```golang
// modules/core/04-channel/types/tx.pb.go 
func _Msg_RecvPacket_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
in := new(MsgRecvPacket)
if err := dec(in); err != nil {
return nil, err
}
if interceptor == nil {
return srv.(MsgServer).RecvPacket(ctx, in)
}
info := &grpc.UnaryServerInfo{
Server:     srv,
FullMethod: "/ibc.core.channel.v1.Msg/RecvPacket",
}
handler := func(ctx context.Context, req interface{}) (interface{}, error) {
return srv.(MsgServer).RecvPacket(ctx, req.(*MsgRecvPacket))
}
return interceptor(ctx, in, info, handler)
}
```

```golang 
// modules/core/keeper/msg_server.go
func (k Keeper) RecvPacket(goCtx context.Context, msg *channeltypes.MsgRecvPacket) (*channeltypes.MsgRecvPacketResponse, error) {
}
```

```
cbs, ok := k.Router.GetRoute(module)
```

```
err = k.ChannelKeeper.RecvPacket(cacheCtx, cap, msg.Packet, msg.ProofCommitment, msg.ProofHeight)
(modules/core/04-channel/keeper/packet.go )

ack := cbs.OnRecvPacket(cacheCtx, msg.Packet, relayer) ( modules/core/keeper/msg_server.go ) 
```

```
-> func (im IBCMiddleware) OnRecvPacket(
ctx sdk.Context,
packet channeltypes.Packet,
relayer sdk.AccAddress,
) ibcexported.Acknowledgement { }  ( x/qtransfer/ibc_module.go )

```

```
	if hook, ok := im.ICS4Middleware.Hooks.(OnRecvPacketOverrideHooks); ok {
		return hook.OnRecvPacketOverride(im, ctx, packet, relayer)
	}

func (h WasmHooks) OnRecvPacketOverride(im IBCMiddleware, ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress) ibcexported.Acknowledgement {..}
(x/qtransfer/wasm_hooks.go)

```

```
func (im IBCModule) OnRecvPacket(
ctx sdk.Context,
packet channeltypes.Packet,
relayer sdk.AccAddress,
) ibcexported.Acknowledgement {} ( modules/apps/transfer/ibc_module.go )  

```
```
func (k Keeper) OnRecvPacket(ctx sdk.Context, packet channeltypes.Packet, data types.FungibleTokenPacketData) error   ( modules/apps/transfer/keeper/relay.go )
```