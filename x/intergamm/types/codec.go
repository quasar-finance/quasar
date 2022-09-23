package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/v9/gamm/pool-models/balancer"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/v9/gamm/types"
	lockuptypes "github.com/quasarlabs/quasarnode/osmosis/v9/lockup/types"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgTestScenario{}, "intergamm/TestScenario", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcJoinPool{}, "intergamm/TransmitIbcJoinPool", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcExitPool{}, "intergamm/TransmitIbcExitPool", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcLockTokens{}, "intergamm/TransmitIbcLockTokens", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcBeginUnlocking{}, "intergamm/TransmitIbcBeginUnlocking", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcJoinSwapExternAmountIn{}, "intergamm/TransmitIbcJoinSwapExternAmountIn", nil)
	cdc.RegisterConcrete(&MsgTransmitIbcExitSwapExternAmountOut{}, "intergammTransmitIbcExitSwapExternAmountOut", nil)

	cdc.RegisterConcrete(&MsgSendToken{}, "intergamm/SendToken", nil)
	cdc.RegisterConcrete(&MsgTransmitICATransfer{}, "intergamm/TransmitICATransfer", nil)
	cdc.RegisterConcrete(&MsgRegisterICAOnZone{}, "intergamm/RegisterICAOnZone", nil)
	cdc.RegisterConcrete(&MsgRegisterICAOnDenomNativeZone{}, "intergamm/RegisterICAOnDenomNativeZone", nil)
	cdc.RegisterConcrete(&MsgSendTokenToICA{}, "intergamm/SendTokenToICA", nil)
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
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgSendToken{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgTransmitICATransfer{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgRegisterICAOnZone{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgRegisterICAOnDenomNativeZone{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgSendTokenToICA{},
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
