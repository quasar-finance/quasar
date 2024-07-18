package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/codec/legacy"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

/*
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
*/

var (
	amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)

func RegisterLegacyAminoCodec(cdc *codec.LegacyAmino) {
	legacy.RegisterAminoMsg(cdc, &MsgCreateDenom{}, "quasar/tokenfactory/create-denom")
	legacy.RegisterAminoMsg(cdc, &MsgMint{}, "quasar/tokenfactory/mint")
	legacy.RegisterAminoMsg(cdc, &MsgBurn{}, "quasar/tokenfactory/burn")
	legacy.RegisterAminoMsg(cdc, &MsgChangeAdmin{}, "quasar/tokenfactory/change-admin")
	legacy.RegisterAminoMsg(cdc, &MsgSetDenomMetadata{}, "osmosis/tokenfactory/set-denom-metadata")
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations(
		(*sdk.Msg)(nil),
		&MsgCreateDenom{},
		&MsgMint{},
		&MsgBurn{},
		&MsgChangeAdmin{},
		&MsgSetDenomMetadata{},
	)
	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

/*
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
*/
