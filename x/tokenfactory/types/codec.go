package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/codec/legacy"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

var (
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)

func RegisterLegacyAminoCodec(cdc *codec.LegacyAmino) {
	legacy.RegisterAminoMsg(cdc, &MsgCreateDenom{}, "quasar/tokenfactory/create-denom")
	legacy.RegisterAminoMsg(cdc, &MsgMint{}, "quasar/tokenfactory/mint")
	legacy.RegisterAminoMsg(cdc, &MsgBurn{}, "quasar/tokenfactory/burn")
	legacy.RegisterAminoMsg(cdc, &MsgChangeAdmin{}, "quasar/tokenfactory/change-admin")
	legacy.RegisterAminoMsg(cdc, &MsgSetDenomMetadata{}, "quasar/tokenfactory/set-denom-metadata")
	legacy.RegisterAminoMsg(cdc, &MsgSetBeforeSendHook{}, "quasar/tokenfactory/set-bef-send-hook")
	legacy.RegisterAminoMsg(cdc, &MsgForceTransfer{}, "quasar/tokenfactory/force-transfer")
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations(
		(*sdk.Msg)(nil),
		&MsgCreateDenom{},
		&MsgMint{},
		&MsgBurn{},
		&MsgChangeAdmin{},
		&MsgSetDenomMetadata{},
		&MsgSetBeforeSendHook{},
		&MsgForceTransfer{},
	)
	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}
