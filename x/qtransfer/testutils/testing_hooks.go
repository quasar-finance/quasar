package testutils

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	ibcexported "github.com/cosmos/ibc-go/v5/modules/core/exported"
	"github.com/quasarlabs/quasarnode/x/qtransfer"
)

var (
	_ qtransfer.Hooks = TestRecvOverrideHooks{}
	_ qtransfer.Hooks = TestRecvBeforeAfterHooks{}
)

type Status struct {
	OverrideRan bool
	BeforeRan   bool
	AfterRan    bool
}

// Recv
type TestRecvOverrideHooks struct{ Status *Status }

func (t TestRecvOverrideHooks) OnRecvPacketOverride(im qtransfer.IBCMiddleware, ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress) ibcexported.Acknowledgement {
	t.Status.OverrideRan = true
	ack := im.App.OnRecvPacket(ctx, packet, relayer)
	return ack
}

type TestRecvBeforeAfterHooks struct{ Status *Status }

func (t TestRecvBeforeAfterHooks) OnRecvPacketBeforeHook(ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress) {
	t.Status.BeforeRan = true
}

func (t TestRecvBeforeAfterHooks) OnRecvPacketAfterHook(ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress, ack ibcexported.Acknowledgement) {
	t.Status.AfterRan = true
}
