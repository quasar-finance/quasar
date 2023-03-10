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

# ACKNOWLEDGEMENT 


func (app *BaseApp) runTx(mode runTxMode, txBytes []byte) {}

result, err = app.runMsgs(runMsgCtx, msgs, mode) {}


baseapp/baseapp.go
func (app *BaseApp) runMsgs(ctx sdk.Context, msgs []sdk.Msg, mode runTxMode) (*sdk.Result, error)



baseapp/msg_service_router.go 

func (msr *MsgServiceRouter) RegisterService(sd *grpc.ServiceDesc, handler interface{})




func _Msg_Acknowledgement_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
in := new(MsgAcknowledgement)
if err := dec(in); err != nil {
return nil, err
}
if interceptor == nil {
return srv.(MsgServer).Acknowledgement(ctx, in)
}
info := &grpc.UnaryServerInfo{
Server:     srv,
FullMethod: "/ibc.core.channel.v1.Msg/Acknowledgement",
}
handler := func(ctx context.Context, req interface{}) (interface{}, error) {
return srv.(MsgServer).Acknowledgement(ctx, req.(*MsgAcknowledgement))
}
return interceptor(ctx, in, info, handler)
}


modules/core/keeper/msg_server.go

// Acknowledgement defines a rpc handler method for MsgAcknowledgement.
func (k Keeper) Acknowledgement(goCtx context.Context, msg *channeltypes.MsgAcknowledgement) {
...


// Perform application logic callback
err = cbs.OnAcknowledgementPacket(ctx, msg.Packet, msg.Acknowledgement, relayer)

.....
}

x/qtransfer/wasm_hooks.go 
func (h WasmHooks) OnAcknowledgementPacketOverride(im IBCMiddleware, ctx sdk.Context, packet channeltypes.Packet, acknowledgement []byte, relayer sdk.AccAddress) error {
err := im.App.OnAcknowledgementPacket(ctx, packet, acknowledgement, relayer) {
}

->
modules/apps/transfer/ibc_module.go 

// OnAcknowledgementPacket implements the IBCModule interface
func (im IBCModule) OnAcknowledgementPacket(
ctx sdk.Context,
packet channeltypes.Packet,
acknowledgement []byte,
relayer sdk.AccAddress,
) error {}

-> 
modules/apps/transfer/keeper/relay.go
func (k Keeper) OnAcknowledgementPacket(ctx sdk.Context, packet channeltypes.Packet, data types.FungibleTokenPacketData, ack channeltypes.Acknowledgement) error {...}



->
modules/apps/transfer/ibc_module.go
if hook, ok := im.ICS4Middleware.Hooks.(OnAcknowledgementPacketOverrideHooks); ok {
**_return hook.OnAcknowledgementPacketOverride(im, ctx, packet, acknowledgement, relayer)**_
}

->
x/qtransfer/ibc_module.go -> 

// OnAcknowledgementPacket implements the IBCMiddleware interface
func (im IBCMiddleware) OnAcknowledgementPacket(
ctx sdk.Context,
packet channeltypes.Packet,
acknowledgement []byte,
relayer sdk.AccAddress,
) 

->
x/qtransfer/wasm_hooks.go

func (h WasmHooks) OnAcknowledgementPacketOverride(im IBCMiddleware, ctx sdk.Context, packet channeltypes.Packet, acknowledgement []byte, relayer sdk.AccAddress) error {
err := im.App.OnAcknowledgementPacket(ctx, packet, acknowledgement, relayer)
if err != nil {
return err
}

}


modules/apps/transfer/ibc_module.go
// OnAcknowledgementPacket implements the IBCMiddleware interface
func (im IBCMiddleware) OnAcknowledgementPacket(
ctx sdk.Context,
packet channeltypes.Packet,
acknowledgement []byte,
relayer sdk.AccAddress,
) error {
