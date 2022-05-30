package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
	osmosiscdc1 "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	osmosiscdc2 "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	osmosiscdc3 "github.com/osmosis-labs/osmosis/v7/x/lockup/types"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgTestScenario{}, "intergamm/TestScenario", nil)
	// this line is used by starport scaffolding # 2

	//
	osmosiscdc1.RegisterLegacyAminoCodec(cdc)
	osmosiscdc2.RegisterLegacyAminoCodec(cdc)
	osmosiscdc3.RegisterCodec(cdc)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTestScenario{},
	)
	// this line is used by starport scaffolding # 3

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)

	osmosiscdc1.RegisterInterfaces(registry)
	osmosiscdc2.RegisterInterfaces(registry)
	osmosiscdc3.RegisterInterfaces(registry)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
