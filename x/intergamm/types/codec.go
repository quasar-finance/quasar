package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
	gammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types/osmosis/v9/gamm"
	gammbalancer "github.com/quasarlabs/quasarnode/x/intergamm/types/osmosis/v9/gamm/pool-models/balancer"
	lockuptypes "github.com/quasarlabs/quasarnode/x/intergamm/types/osmosis/v9/lockup"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgTestScenario{}, "intergamm/TestScenario", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcJoinPool{}, "intergamm/TransmitIbcJoinPool", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcExitPool{}, "intergamm/TransmitIbcExitPool", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcLockTokens{}, "intergamm/TransmitIbcLockTokens", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcBeginUnlocking{}, "intergamm/TransmitIbcBeginUnlocking", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcJoinSwapExternAmountIn{}, "intergamm/TransmitIbcJoinSwapExternAmountIn", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcExitSwapExternAmountOut{}, "intergammTransmitIbcExitSwapExternAmountOut", nil)

	// this line is used by starport scaffolding # 2

	gammtypes.RegisterLegacyAminoCodec(cdc)
	gammbalancer.RegisterLegacyAminoCodec(cdc)
	lockuptypes.RegisterCodec(cdc)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTestScenario{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTransmitIbcJoinPool{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTransmitIbcExitPool{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTransmitIbcJoinPool{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTransmitIbcBeginUnlocking{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTransmitIbcJoinSwapExternAmountIn{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTransmitIbcExitSwapExternAmountOut{},
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
