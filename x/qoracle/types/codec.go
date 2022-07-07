package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgCreatePoolPosition{}, "qoracle/CreatePoolPosition", nil)
	cdc.RegisterConcrete(&MsgUpdatePoolPosition{}, "qoracle/UpdatePoolPosition", nil)
	cdc.RegisterConcrete(&MsgDeletePoolPosition{}, "qoracle/DeletePoolPosition", nil)
	cdc.RegisterConcrete(&MsgCreatePoolRanking{}, "qoracle/CreatePoolRanking", nil)
	cdc.RegisterConcrete(&MsgUpdatePoolRanking{}, "qoracle/UpdatePoolRanking", nil)
	cdc.RegisterConcrete(&MsgDeletePoolRanking{}, "qoracle/DeletePoolRanking", nil)
	cdc.RegisterConcrete(&MsgCreatePoolSpotPrice{}, "qoracle/CreatePoolSpotPrice", nil)
	cdc.RegisterConcrete(&MsgUpdatePoolSpotPrice{}, "qoracle/UpdatePoolSpotPrice", nil)
	cdc.RegisterConcrete(&MsgDeletePoolSpotPrice{}, "qoracle/DeletePoolSpotPrice", nil)
	cdc.RegisterConcrete(&MsgCreatePoolInfo{}, "qoracle/CreatePoolInfo", nil)
	cdc.RegisterConcrete(&MsgUpdatePoolInfo{}, "qoracle/UpdatePoolInfo", nil)
	cdc.RegisterConcrete(&MsgDeletePoolInfo{}, "qoracle/DeletePoolInfo", nil)
	cdc.RegisterConcrete(&MsgStablePrice{}, "qoracle/StablePrice", nil)
	// this line is used by starport scaffolding # 2
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*CoinRatesCallDataI)(nil), &CoinRatesCallData{})
	registry.RegisterImplementations((*CoinRatesResultI)(nil), &CoinRatesResult{})
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgCreatePoolPosition{},
		&MsgUpdatePoolPosition{},
		&MsgDeletePoolPosition{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgCreatePoolRanking{},
		&MsgUpdatePoolRanking{},
		&MsgDeletePoolRanking{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgCreatePoolSpotPrice{},
		&MsgUpdatePoolSpotPrice{},
		&MsgDeletePoolSpotPrice{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgCreatePoolInfo{},
		&MsgUpdatePoolInfo{},
		&MsgDeletePoolInfo{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgStablePrice{},
	)
	// this line is used by starport scaffolding # 3

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
