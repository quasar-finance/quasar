package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgRequestDeposit{}, "qbank/RequestDeposit", nil)
	cdc.RegisterConcrete(&MsgRequestWithdraw{}, "qbank/RequestWithdraw", nil)
	cdc.RegisterConcrete(&MsgClaimRewards{}, "qbank/ClaimRewards", nil)
	cdc.RegisterConcrete(&MsgRequestWithdrawAll{}, "qbank/RequestWithdrawAll", nil)
	// this line is used by starport scaffolding # 2
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgRequestDeposit{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgRequestWithdraw{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgClaimRewards{},
	)
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgRequestWithdrawAll{},
	)
	// this line is used by starport scaffolding # 3

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
