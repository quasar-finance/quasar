package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/gamm/types"
	lockuptypes "github.com/quasarlabs/quasarnode/osmosis/lockup/types"
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

	// Register concrete types of osmosis gamm module
	cdc.RegisterConcrete(&gammtypes.MsgJoinPool{}, "osmosis/gamm/join-pool", nil)
	cdc.RegisterConcrete(&gammtypes.MsgExitPool{}, "osmosis/gamm/exit-pool", nil)
	cdc.RegisterConcrete(&gammtypes.MsgSwapExactAmountIn{}, "osmosis/gamm/swap-exact-amount-in", nil)
	cdc.RegisterConcrete(&gammtypes.MsgSwapExactAmountOut{}, "osmosis/gamm/swap-exact-amount-out", nil)
	cdc.RegisterConcrete(&gammtypes.MsgJoinSwapExternAmountIn{}, "osmosis/gamm/join-swap-extern-amount-in", nil)
	cdc.RegisterConcrete(&gammtypes.MsgJoinSwapShareAmountOut{}, "osmosis/gamm/join-swap-share-amount-out", nil)
	cdc.RegisterConcrete(&gammtypes.MsgExitSwapExternAmountOut{}, "osmosis/gamm/exit-swap-extern-amount-out", nil)
	cdc.RegisterConcrete(&gammtypes.MsgExitSwapShareAmountIn{}, "osmosis/gamm/exit-swap-share-amount-in", nil)

	// Register concrete types of osmosis gamm balancer module
	cdc.RegisterConcrete(&gammbalancer.Pool{}, "osmosis/gamm/BalancerPool", nil)
	cdc.RegisterConcrete(&gammbalancer.MsgCreateBalancerPool{}, "osmosis/gamm/create-balancer-pool", nil)
	cdc.RegisterConcrete(&gammbalancer.PoolParams{}, "osmosis/gamm/BalancerPoolParams", nil)

	// Register concrete types of osmosis lockup module
	cdc.RegisterConcrete(&lockuptypes.MsgLockTokens{}, "osmosis/lockup/lock-tokens", nil)
	cdc.RegisterConcrete(&lockuptypes.MsgBeginUnlockingAll{}, "osmosis/lockup/begin-unlock-tokens", nil)
	cdc.RegisterConcrete(&lockuptypes.MsgBeginUnlocking{}, "osmosis/lockup/begin-unlock-period-lock", nil)
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

	// Register osmosis gamm module interfaces
	registry.RegisterImplementations(
		(*sdk.Msg)(nil),
		&gammtypes.MsgJoinPool{},
		&gammtypes.MsgExitPool{},
		&gammtypes.MsgSwapExactAmountIn{},
		&gammtypes.MsgSwapExactAmountOut{},
		&gammtypes.MsgJoinSwapExternAmountIn{},
		&gammtypes.MsgJoinSwapShareAmountOut{},
		&gammtypes.MsgExitSwapExternAmountOut{},
		&gammtypes.MsgExitSwapShareAmountIn{},
	)

	// Register osmosis gamm balancer module interfaces
	registry.RegisterImplementations(
		(*sdk.Msg)(nil),
		&gammbalancer.MsgCreateBalancerPool{},
	)
	registry.RegisterImplementations(
		(*proto.Message)(nil),
		&gammbalancer.PoolParams{},
	)

	// Register osmosis lockup module interfaces
	registry.RegisterImplementations(
		(*sdk.Msg)(nil),
		&lockuptypes.MsgLockTokens{},
		&lockuptypes.MsgBeginUnlockingAll{},
		&lockuptypes.MsgBeginUnlocking{},
	)

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
