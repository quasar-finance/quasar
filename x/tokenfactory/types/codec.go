package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authzcodec "github.com/cosmos/cosmos-sdk/x/authz/codec"
	// this line is used by starport scaffolding # 1
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgCreateDenom{}, "quasar/tokenfactory/create-denom", nil)
	cdc.RegisterConcrete(&MsgMint{}, "quasar/tokenfactory/mint", nil)
	cdc.RegisterConcrete(&MsgBurn{}, "quasar/tokenfactory/burn", nil)
	cdc.RegisterConcrete(&MsgChangeAdmin{}, "quasar/tokenfactory/change-admin", nil)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations(
		(*sdk.Msg)(nil),
		&MsgCreateDenom{},
		&MsgMint{},
		&MsgBurn{},
		&MsgChangeAdmin{},
	)
	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

var (
	amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)

func init() {
	RegisterCodec(amino)
	// Register all Amino interfaces and concrete types on the authz Amino codec so that this can later be
	// used to properly serialize MsgGrant and MsgExec instances
	// Note: these 3 are inlines from authz/codec in 0.46 so we can be compatible with 0.45
	sdk.RegisterLegacyAminoCodec(amino)
	// cryptocodec.RegisterCrypto(amino)
	RegisterCodec(authzcodec.Amino)
	// codec.RegisterEvidences(amino)

	amino.Seal()
}
