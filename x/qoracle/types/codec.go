package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	// cdc.RegisterConcrete(&MsgAddDenomSymbolMappings{}, "qoracle/MsgAddDenomSymbolMappings", nil)
	// cdc.RegisterConcrete(&MsgRemoveDenomSymbolMappings{}, "qoracle/MsgRemoveDenomSymbolMappings", nil)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {

	// registry.RegisterImplementations((*sdk.Msg)(nil),
	//	&MsgAddDenomSymbolMappings{},
	//	&MsgRemoveDenomSymbolMappings{},
	// )

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
