package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
	osmosiscdc1 "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	osmosiscdc2 "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgRegisterAccount{}, "intergamm/RegisterAccount", nil)
	cdc.RegisterConcrete(&MsgCreatePool{}, "intergamm/CreatePool", nil)
	cdc.RegisterConcrete(&MsgJoinPool{}, "intergamm/JoinPool", nil)
	cdc.RegisterConcrete(&MsgExitPool{}, "intergamm/ExitPool", nil)
	cdc.RegisterConcrete(&MsgIbcTransfer{}, "intergamm/IbcTransfer", nil)
	cdc.RegisterConcrete(&MsgForwardIbcTransfer{}, "intergamm/ForwardIbcTransfer", nil)
	// this line is used by starport scaffolding # 2

	//
	osmosiscdc1.RegisterLegacyAminoCodec(cdc)
	osmosiscdc2.RegisterLegacyAminoCodec(cdc)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgRegisterAccount{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgCreatePool{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgJoinPool{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgExitPool{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgIbcTransfer{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgForwardIbcTransfer{},
	)
	// this line is used by starport scaffolding # 3

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)

	//
	osmosiscdc1.RegisterInterfaces(registry)
	osmosiscdc2.RegisterInterfaces(registry)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
