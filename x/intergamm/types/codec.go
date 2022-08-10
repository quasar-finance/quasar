package types

import (
	gammbalancer "github.com/abag/quasarnode/osmosis/v9/gamm/pool-models/balancer"
	gammtypes "github.com/abag/quasarnode/osmosis/v9/gamm/types"
	lockuptypes "github.com/abag/quasarnode/osmosis/v9/lockup/types"
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgTestScenario{}, "intergamm/TestScenario", nil)
	// this line is used by starport scaffolding # 2

	gammtypes.RegisterLegacyAminoCodec(cdc)
	gammbalancer.RegisterLegacyAminoCodec(cdc)
	lockuptypes.RegisterCodec(cdc)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTestScenario{},
	)
	// this line is used by starport scaffolding # 3

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)

	gammtypes.RegisterInterfaces(registry)
	gammbalancer.RegisterInterfaces(registry)
	lockuptypes.RegisterInterfaces(registry)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
