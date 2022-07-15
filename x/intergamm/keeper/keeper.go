package keeper

import (
	"fmt"
	"strings"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	connectiontypes "github.com/cosmos/ibc-go/v3/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
	ibctmtypes "github.com/cosmos/ibc-go/v3/modules/light-clients/07-tendermint/types"
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
	connectionKeeper    types.ConnectionKeeper
	clientKeeper        types.ClientKeeper
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
	connectionKeeper types.ConnectionKeeper,
	clientKeeper types.ClientKeeper,
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
		connectionKeeper:    connectionKeeper,
		clientKeeper:        clientKeeper,
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

func (k Keeper) GetChannelKeeper(ctx sdk.Context) types.ChannelKeeper {
	return k.channelKeeper
}

// GetIntermediateReceiver retrieve the intermediate receiver address in the middle chain
// for packet forwarding.
// Arguments connectionID is the connection id of the next intermediate chain
// NOTE - We can very well have local zone -id (chainid/connectionid) to easy the calculations
func (k Keeper) GetIntermediateReceiver(ctx sdk.Context,
	connectionID string) string {
	intr_rcvrs := k.IntrRcvrs(ctx)
	for _, v := range intr_rcvrs {
		if v.ZoneInfo.ConnectionId == connectionId {
			return v.RcvrAddress
		}
	}
	return ""
	// return "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu" // alice on cosmos
}

func (k Keeper) GetAllConnections(ctx sdk.Context) (connections []connectiontypes.IdentifiedConnection) {
	return k.connectionKeeper.GetAllConnections(ctx)
}

func (k Keeper) GetChainID(ctx sdk.Context, connectionID string) (string, error) {
	conn, found := k.connectionKeeper.GetConnection(ctx, connectionID)
	if !found {
		return "", fmt.Errorf("invalid connection id, \"%s\" not found", connectionID)
	}
	clientState, found := k.clientKeeper.GetClientState(ctx, conn.ClientId)
	if !found {
		return "", fmt.Errorf("client id \"%s\" not found for connection \"%s\"", conn.ClientId, connectionID)
	}
	client, ok := clientState.(*ibctmtypes.ClientState)
	if !ok {
		return "", fmt.Errorf("invalid client state for client \"%s\" on connection \"%s\"", conn.ClientId, connectionID)
	}

	return client.ChainId, nil
}

// getConnectionId returns the connection identifier to osmosis from intergamm module
func (k Keeper) GetConnectionId(ctx sdk.Context, inChainID string) (string, bool) {

	logger := k.Logger(ctx)
	for _, c := range k.GetAllConnections(ctx) {
		logger.Info("GetConnectionId", "Connection", c)
		chainID, err := k.GetChainID(ctx, c.Id)
		if err != nil {
			logger.Info("GetConnectionId",
				"Connection", c,
				"GetChainID failed.", err)
		} else {
			logger.Info("GetConnectionId",
				"Connection", c,
				"chainID", chainID)
			if chainID == inChainID {
				return c.Id, true
			}
		}
	}
	return "", false
}
func (k Keeper) SetPortDetail(ctx sdk.Context, pi types.PortInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PortInfoKBP)
	destinationChain, _ := k.GetChainID(ctx, pi.ConnectionID)
	key := types.CreateChainIDPortIDKey(destinationChain, pi.PortID)
	b := k.cdc.MustMarshal(&pi)
	store.Set(key, b)
}

func (k Keeper) GetPortDetail(ctx sdk.Context, destinationChain, portID string) (pi types.PortInfo, found bool) {
	logger := k.Logger(ctx)
	logger.Info("GetPortDetail", "portID", portID, "destinationChain", destinationChain)
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PortInfoKBP)
	key := types.CreateChainIDPortIDKey(destinationChain, portID)
	logger.Info("GetPortDetail", "key", string(key))
	b := store.Get(key)
	if b == nil {
		logger.Info("GetPortDetail key not found", "key", string(key))
		return pi, false
	}
	k.cdc.MustUnmarshal(b, &pi)
	return pi, true
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

func (k Keeper) IsICARegistered(ctx sdk.Context, connectionID, owner string) (string, bool) {
	portID, err := icatypes.NewControllerPortID(owner)
	if err != nil {
		return "", false
	}
	return k.icaControllerKeeper.GetInterchainAccountAddress(ctx, connectionID, portID)
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
	// destinationChain string, // TODO - To be used for cross validation
	destinationLocalZoneId string,
	sender sdk.AccAddress,
	receiver string,
	coin sdk.Coin) (uint64, error) {
	logger := k.Logger(ctx)
	logger.Info("SendToken",
		"destinationLocalZoneId", destinationLocalZoneId,
		"sender", sender,
		"receiver", receiver,
		"coin", coin,
	)

	var is_one_hop bool
	var destinationChain string
	var fwdChannelID string
	if intrZone, found := k.DestToIntrZoneMap(ctx)[destinationLocalZoneId]; !found {
		// DestToIntrZoneMap does not have a corresponding entry.
		// This could mean that; destination zone is the intrZone for this case.
		// And this is normal one hop token tranfer.

		// Verifying is intr zone exist
		intrRcvrs := k.IntrRcvrs(ctx)
		for _, intrRcvr := range intrRcvrs {
			if destinationLocalZoneId == intrRcvr.ZoneInfo.LocalZoneId {
				is_one_hop = true
				destinationChain = intrRcvr.ZoneInfo.ChainId
			}
		}
	} else {
		// Packet forwarding case
		intrRcvrs := k.IntrRcvrs(ctx)
		for _, intrRcvr := range intrRcvrs {
			if intrZone == intrRcvr.ZoneInfo.LocalZoneId {
				if nextZone, found := intrRcvr.NextZoneRouteMap[destinationLocalZoneId]; found {
					fwdChannelID = nextZone.TransferChannelId
					destinationChain = nextZone.ChainId
				} else {
					return 0, fmt.Errorf("packet forwarding channel not found for intr zone %s,next zone %s",
						intrZone, destinationLocalZoneId)
				}
			}
		}
	}

	if destinationChain == "" {
		return 0, fmt.Errorf("destination chain not found for zone %s", destinationLocalZoneId)
	}

	if !is_one_hop && fwdChannelID == "" {
		return 0, fmt.Errorf("packet forwarding channel is empty for two hop token transfer to dest zone id %s", destinationLocalZoneId)
	}

	pi, found := k.GetPortDetail(ctx, destinationChain, "transfer")
	if !found {
		return 0, fmt.Errorf("token transfer to osmosis not available right now, port info not found")
	}

	connectionTimeout := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp
	transferTimeoutHeight := clienttypes.Height{RevisionNumber: 0, RevisionHeight: 0}
	if is_one_hop {
		// if coin.Denom == "uqsr" { // Native gov token
		// Support for all non-ibc tokens should be added here in future so to support
		// other vaults native tokens with cosmwasm.

		return k.TransferIbcTokens(ctx, "transfer",
			pi.ChannelID, coin,
			sender, receiver,
			transferTimeoutHeight, connectionTimeout)
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
		logger.Info("SendToken",
			"denomTrace", denomTrace,
		)

		return k.ForwardTransferIbcTokens(ctx, "transfer", pi.ChannelID, coin,
			sender, "transfer",
			fwdChannelID,
			"cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu", // This hardcoded address is TODO.
			receiver, transferTimeoutHeight, connectionTimeout)

		/*
			switch denomTrace.BaseDenom {
			case "uosmo": // no middle chain involves

				return k.TransferIbcTokens(ctx, "transfer", pi.ChannelID, coin,
					sender, receiver, transferTimeoutHeight, connectionTimeout)

			case "uatom": // middlechain is comsos-hub
				// TODO - Get the middle chain intermediate address
				fwdChannelID := k.OsmoTokenTransferChannels(ctx)[destinationChain]
				return k.ForwardTransferIbcTokens(ctx, "transfer", pi.ChannelID, coin,
					sender, "transfer",
					fwdChannelID,

					"cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu", // This hardcoded address is TODO.
					receiver, transferTimeoutHeight, connectionTimeout)

			default:
				return 0, sdkerrors.Wrap(ibctransfertypes.ErrInvalidDenomForTransfer, hexHash)
			}
		*/
	}

	return 0, sdkerrors.Wrap(ibctransfertypes.ErrInvalidDenomForTransfer, "unrecognized ibc denom")
}
