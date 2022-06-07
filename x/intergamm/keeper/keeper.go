package keeper

import (
	"fmt"
	"strings"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
)

var (
	// Timeout timestamp following Transfer timestamp default
	DefaultSendTxRelativeTimeoutTimestamp = ibctransfertypes.DefaultRelativePacketTimeoutTimestamp
)

type GammHooks struct {
	IbcTransfer IbcTransferHooks
	Osmosis     OsmosisHooks
}

type Keeper struct {
	cdc                 codec.BinaryCodec
	storeKey            sdk.StoreKey
	memKey              sdk.StoreKey
	scopedKeeper        capabilitykeeper.ScopedKeeper
	channelKeeper       types.ChannelKeeper
	icaControllerKeeper types.ICAControllerKeeper
	ibcTransferKeeper   types.IBCTransferKeeper
	paramstore          paramtypes.Subspace

	Hooks GammHooks
}

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey sdk.StoreKey,
	scopedKeeper capabilitykeeper.ScopedKeeper,
	channelKeeper types.ChannelKeeper,
	iaKeeper types.ICAControllerKeeper,
	transferKeeper types.IBCTransferKeeper,
	ps paramtypes.Subspace,
) *Keeper {
	// set KeyTable if it has not already been set
	if !ps.HasKeyTable() {
		ps = ps.WithKeyTable(types.ParamKeyTable())
	}

	return &Keeper{
		cdc:                 cdc,
		storeKey:            storeKey,
		memKey:              memKey,
		scopedKeeper:        scopedKeeper,
		channelKeeper:       channelKeeper,
		icaControllerKeeper: iaKeeper,
		ibcTransferKeeper:   transferKeeper,
		paramstore:          ps,

		Hooks: GammHooks{
			IbcTransfer: IbcTransferHooks{},
			Osmosis:     OsmosisHooks{},
		},
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

// ClaimCapability claims the channel capability passed via the OnOpenChanInit callback
func (k *Keeper) ClaimCapability(ctx sdk.Context, cap *capabilitytypes.Capability, name string) error {
	return k.scopedKeeper.ClaimCapability(ctx, cap, name)
}

func (k *Keeper) NewCapability(ctx sdk.Context, name string) (*capabilitytypes.Capability, error) {
	return k.scopedKeeper.NewCapability(ctx, name)
}

func (k Keeper) RegisterInterchainAccount(ctx sdk.Context, connectionID, owner string) error {
	return k.icaControllerKeeper.RegisterInterchainAccount(ctx, connectionID, owner)
}

func (k Keeper) sendTxOverIca(ctx sdk.Context, owner, connectionId string, msgs []sdk.Msg, timeoutTimestamp uint64) (uint64, error) {
	portID, err := icatypes.NewControllerPortID(owner)
	if err != nil {
		return 0, err
	}
	channelID, found := k.icaControllerKeeper.GetActiveChannelID(ctx, connectionId, portID)
	if !found {
		return 0, sdkerrors.Wrapf(icatypes.ErrActiveChannelNotFound, "failed to retrieve active channel for port %s", portID)
	}
	chanCap, found := k.scopedKeeper.GetCapability(ctx, host.ChannelCapabilityPath(portID, channelID))
	if !found {
		return 0, sdkerrors.Wrap(channeltypes.ErrChannelCapabilityNotFound, "module does not own channel capability")
	}

	data, err := icatypes.SerializeCosmosTx(k.cdc, msgs)
	if err != nil {
		return 0, err
	}

	packetData := icatypes.InterchainAccountPacketData{
		Type: icatypes.EXECUTE_TX,
		Data: data,
	}

	timeoutNano := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp
	seq, err := k.icaControllerKeeper.SendTx(ctx, chanCap, connectionId, portID, packetData, timeoutNano)
	if err != nil {
		return 0, err
	}

	k.Logger(ctx).Info("sendTx ICA", "seq", seq)

	return seq, nil
}

// SendToken will send token to the destination chain ( assuming it is only osmosis in the beggining)
// TODO - Hardcoded values of port, channel, and connection to be determined by routing logic or config
// Vault should send validated values of address, and coin
// Current version support only uqsr as native non ibc token denom
// NOTE - This method to be used till automated routing logic is in place.
func (k Keeper) SendToken(ctx sdk.Context,
	destinationChain string, // TODO - To be used for cross validation
	sender sdk.AccAddress,
	receiver string,
	coin sdk.Coin) (uint64, error) {

	connectionTimeout := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp
	transferTimeoutHeight := clienttypes.Height{RevisionNumber: 0, RevisionHeight: 0}
	if coin.Denom == "uqsr" { // Native gov token
		// Support for all non-ibc tokens should be added here in future so to support
		// other vaults native tokens with cosmwasm.
		return k.TransferIbcTokens(ctx, "transfer", "channel-0", coin,
			sender, receiver, transferTimeoutHeight, connectionTimeout)
	}

	// IBC denom validations in case vaults are giving wrong arguments
	ibcPrefix := ibctransfertypes.DenomPrefix + "/"
	if strings.HasPrefix(coin.Denom, ibcPrefix) {
		hexHash := coin.Denom[len(ibcPrefix):]
		hash, err := ibctransfertypes.ParseHexHash(hexHash)
		if err != nil {
			return 0, sdkerrors.Wrap(ibctransfertypes.ErrInvalidDenomForTransfer, err.Error())
		}

		denomTrace, ok := k.ibcTransferKeeper.GetDenomTrace(ctx, hash)
		if !ok {
			return 0, sdkerrors.Wrap(ibctransfertypes.ErrTraceNotFound, hexHash)
		}

		// TODO - Automated mapping from base denom, and destination chain to <port, channel, connection, middle address>
		// to be determined. Either using some config or other ways.
		// NOTE - Automated Routing PR for intergamm is in progress.
		switch denomTrace.BaseDenom {
		case "uosmo": // no middle chain involves
			return k.TransferIbcTokens(ctx, "transfer", "channel-0", coin,
				sender, receiver, transferTimeoutHeight, connectionTimeout)

		case "uatom": // middlechain is comsos-hub
			// TODO - Get the middle chain intermediate address
			return k.ForwardTransferIbcTokens(ctx, "transfer", "channel-0", coin,
				sender, "transfer", "channel-1", "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu",
				receiver, transferTimeoutHeight, connectionTimeout)

		default:
			return 0, sdkerrors.Wrap(ibctransfertypes.ErrInvalidDenomForTransfer, hexHash)
		}
	}

	return 0, sdkerrors.Wrap(ibctransfertypes.ErrInvalidDenomForTransfer, "unrecognized ibc denom")
}
