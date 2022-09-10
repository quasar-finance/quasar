package keeper

import (
	"errors"
	"fmt"

	"github.com/tendermint/tendermint/libs/log"

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
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
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

func (k Keeper) RegisterICAOnZoneId(ctx sdk.Context, zoneId, owner string) error {
	nativeZoneInfo, found := k.CompleteZoneInfoMap(ctx)[zoneId]
	if !found {
		return fmt.Errorf("error: zone info for zone ID '%s' not specified", zoneId)
	}
	return k.RegisterInterchainAccount(ctx, nativeZoneInfo.ZoneRouteInfo.ConnectionId, owner)
}

func (k Keeper) RegisterICAOnDenomNativeZone(ctx sdk.Context, denom, owner string) error {
	nativeZoneId, found := k.DenomToNativeZoneIdMap(ctx)[denom]
	if !found {
		return fmt.Errorf("error: native zone ID of denom '%s' not specified", denom)
	}
	return k.RegisterICAOnZoneId(ctx, nativeZoneId, owner)
}

func (k Keeper) IsICARegistered(ctx sdk.Context, connectionID, owner string) (string, bool) {
	portID, err := icatypes.NewControllerPortID(owner)
	if err != nil {
		return "", false
	}
	return k.icaControllerKeeper.GetInterchainAccountAddress(ctx, connectionID, portID)
}

func (k Keeper) IsICACreatedOnZoneId(ctx sdk.Context, zoneId, owner string) (string, bool) {
	if zoneId == types.QuasarZoneId {
		return owner, true
	}
	zoneInfo, found := k.CompleteZoneInfoMap(ctx)[zoneId]
	if !found {
		return "", false
	}
	return k.IsICARegistered(ctx, zoneInfo.ZoneRouteInfo.ConnectionId, owner)
}

func (k Keeper) IsICACreatedOnDenomNativeZone(ctx sdk.Context, denom, owner string) (string, bool) {
	nativeZoneId, found := k.DenomToNativeZoneIdMap(ctx)[denom]
	if !found {
		return "", false
	}
	return k.IsICACreatedOnZoneId(ctx, nativeZoneId, owner)
}

// RegisterOrReturnICA returns the address of ICA if it exists, if not attempts to create it and then returns its address.
// If unsuccessful returns an error.
func (k Keeper) RegisterOrReturnICA(ctx sdk.Context, connectionId, owner string) (string, error) {
	if addr, found := k.IsICARegistered(ctx, connectionId, owner); found {
		return addr, nil
	} else {
		logger := k.Logger(ctx)
		logger.Info("RegisterOrReturnICA", fmt.Sprintf("no ICA owned by %s found on IBC connection %s, attempting creation", owner, connectionId))
		if err := k.RegisterInterchainAccount(ctx, connectionId, owner); err != nil {
			return "", err
		}
		if addr, found := k.IsICARegistered(ctx, connectionId, owner); found {
			return addr, nil
		} else {
			return "", errors.New("unexpected error: RegisterInterchainAccount returned no error, but ICA still can't be found")
		}
	}
}

// TODO timeoutTimestamp is ignored here and defaults to DefaultSendTxRelativeTimeoutTimestamp, which is ~10 seconds.
//  timeoutTimestamp should probably be used at some point
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

func (k Keeper) sendTxOverIca2(ctx sdk.Context, connectionId, portId, channelId string, msgs []sdk.Msg, timeoutTimestamp uint64) (uint64, error) {
	chanCap, found := k.scopedKeeper.GetCapability(ctx, host.ChannelCapabilityPath(portId, channelId))
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
	seq, err := k.icaControllerKeeper.SendTx(ctx, chanCap, connectionId, portId, packetData, timeoutNano)
	if err != nil {
		return 0, err
	}

	k.Logger(ctx).Info("sendTx ICA", "seq", seq)

	return seq, nil
}

func (k Keeper) GetZoneInfo(ctx sdk.Context, zoneId string) (zoneInfo types.ZoneCompleteInfo, found bool) {
	zoneInfo, found = k.CompleteZoneInfoMap(ctx)[zoneId]
	return
}

func (k Keeper) GetNativeZoneInfo(ctx sdk.Context, denom string) (types.ZoneCompleteInfo, bool) {
	logger := k.Logger(ctx)
	if nativeZoneId, found := k.DenomToNativeZoneIdMap(ctx)[denom]; found {
		if nativeZoneInfo, found := k.GetZoneInfo(ctx, nativeZoneId); found {
			return nativeZoneInfo, true
		} else {
			logger.Info("GetNativeZoneInfo", fmt.Sprintf("warning: zone info of native zone ID '%s' (for denom '%s') is not found in CompleteZoneInfoMap.", nativeZoneId, denom))
		}
	}
	return types.ZoneCompleteInfo{}, false
}

// SendToken will send token to the destination chain
// Vault should send validated values of address, and coin
func (k Keeper) SendToken(ctx sdk.Context,
	// destinationChain string, // TODO - To be used for cross validation
	destZoneId string,
	sender sdk.AccAddress,
	receiver string,
	coin sdk.Coin) (uint64, error) {
	logger := k.Logger(ctx)
	logger.Info("SendToken",
		"destinationLocalZoneId", destZoneId,
		"sender", sender,
		"receiver", receiver,
		"coin", coin,
	)

	connectionTimeout := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp
	transferTimeoutHeight := clienttypes.Height{RevisionNumber: 0, RevisionHeight: 0}
	nativeZoneId, found := k.DenomToNativeZoneIdMap(ctx)[coin.Denom]
	if !found {
		logger.Error("SendToken", fmt.Sprintf("error: native zone ID of denom '%s' not specified", coin.Denom))
		return 0, errors.New("error: unsupported denom")
	}
	if nativeZoneId == types.QuasarZoneId || nativeZoneId == destZoneId {
		logger.Info("SendToken", "direct transfer")
		// direct transfer
		destZoneInfo, found := k.GetZoneInfo(ctx, destZoneId)
		if !found {
			msg := fmt.Sprintf("error: destination zone info for zone ID '%s' not found in CompleteZoneInfoMap for direct transfer of %s",
				destZoneId, coin.String())
			logger.Error("SendToken", msg)
			return 0, errors.New(msg)
		}
		return k.TransferIbcTokens(ctx,
			destZoneInfo.ZoneRouteInfo.PortId,
			destZoneInfo.ZoneRouteInfo.ChannelId,
			coin,
			sender, receiver,
			transferTimeoutHeight, connectionTimeout)
	} else {
		// forwarding transfer
		logger.Info("SendToken", "forwarding transfer")
		nativeZoneInfo, found := k.GetZoneInfo(ctx, nativeZoneId)
		if !found {
			err := fmt.Errorf("error: zone info for zone ID '%s' not specified", nativeZoneId)
			logger.Error("SendToken", err)
			return 0, err
		}
		// destFromNativeInfo contains IBC info needed to reach destination zone from the native zone.
		destFromNativeInfo, found := nativeZoneInfo.NextZoneRouteMap[destZoneId]
		if !found {
			msg := fmt.Sprintf("error: destination zone info for zone ID '%s' not found in NextZoneRouteMap of native zone with ID '%s' for forwarding transfer of %s",
				destZoneId, nativeZoneInfo.ZoneRouteInfo.CounterpartyZoneId, coin.String())
			logger.Error("SendToken", msg)
			return 0, errors.New(msg)
		}

		nativeIcaAddr, found := k.IsICARegistered(ctx, nativeZoneInfo.ZoneRouteInfo.ConnectionId, sender.String())
		if !found {
			msg := fmt.Sprintf("error: interchain account on native zone (zone ID '%s') for forwarding transfer of %s",
				nativeZoneId, coin.String())
			logger.Error("SendToken", msg)
			return 0, errors.New(msg)
		}

		return k.ForwardTransferIbcTokens(ctx,
			nativeZoneInfo.ZoneRouteInfo.PortId,
			nativeZoneInfo.ZoneRouteInfo.ChannelId,
			coin,
			sender,
			destFromNativeInfo.PortId,
			destFromNativeInfo.ChannelId,
			nativeIcaAddr,
			receiver,
			transferTimeoutHeight, connectionTimeout)
	}
}
