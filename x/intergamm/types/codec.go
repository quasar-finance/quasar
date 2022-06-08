package types

import (
	gammtypes "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/gamm"
	gammbalancer "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/gamm/pool-models/balancer"
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
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTestScenario{},
	)
	// this line is used by starport scaffolding # 3

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)

	gammtypes.RegisterInterfaces(registry)
	gammbalancer.RegisterInterfaces(registry)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
