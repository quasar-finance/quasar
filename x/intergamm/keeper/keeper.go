package keeper

import (
	"fmt"

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
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

var (
	// Timeout timestamp following Transfer timestamp default
	DefaultSendTxRelativeTimeoutTimestamp = ibctransfertypes.DefaultRelativePacketTimeoutTimestamp
)

type GammHooks struct {
	Osmosis OsmosisHooks
}

type Keeper struct {
	cdc                 codec.BinaryCodec
	storeKey            sdk.StoreKey
	memKey              sdk.StoreKey
	scopedKeeper        capabilitykeeper.ScopedKeeper
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
		icaControllerKeeper: iaKeeper,
		ibcTransferKeeper:   transferKeeper,
		paramstore:          ps,

		Hooks: GammHooks{
			Osmosis: OsmosisHooks{},
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

func (k Keeper) TransmitIbcCreatePool(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolParams *gammbalancer.PoolParams,
	poolAssets []gammtypes.PoolAsset,
	futurePoolGovernor string) error {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return err
	}

	msgs := []sdk.Msg{
		&gammbalancer.MsgCreateBalancerPool{
			Sender:             iaResp.InterchainAccountAddress,
			PoolParams:         poolParams,
			PoolAssets:         poolAssets,
			FuturePoolGovernor: futurePoolGovernor,
		},
	}
	return k.sendTx(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcJoinPool(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	shareOutAmount sdk.Int,
	tokenInMaxs []sdk.Coin) error {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgJoinPool{
			Sender:         iaResp.InterchainAccountAddress,
			PoolId:         poolId,
			ShareOutAmount: shareOutAmount,
			TokenInMaxs:    tokenInMaxs,
		},
	}
	return k.sendTx(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcExitPool(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolId uint64,
	shareInAmount sdk.Int,
	tokenOutMins []sdk.Coin) error {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return err
	}

	msgs := []sdk.Msg{
		&gammtypes.MsgExitPool{
			Sender:        iaResp.InterchainAccountAddress,
			PoolId:        poolId,
			ShareInAmount: shareInAmount,
			TokenOutMins:  tokenOutMins,
		},
	}
	return k.sendTx(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

func (k Keeper) TransmitIbcTransfer(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	transferPort, transferChannel string,
	token sdk.Coin,
	receiver string,
	transferTimeoutHeight ibcclienttypes.Height,
	transferTimeoutTimestamp uint64) error {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return err
	}

	msgs := []sdk.Msg{
		&ibctransfertypes.MsgTransfer{
			SourcePort:       transferPort,
			SourceChannel:    transferChannel,
			Token:            token,
			Sender:           iaResp.InterchainAccountAddress,
			Receiver:         receiver,
			TimeoutHeight:    transferTimeoutHeight,
			TimeoutTimestamp: transferTimeoutTimestamp,
		},
	}
	return k.sendTx(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

// TransmitForwardIbcTransfer sends a special case of ibc transfer message that will be forwarded to the destination chain through a middle chain.
// fwdTransferPort and fwdTransferChannel are the port and channel to destination chain on the middle chain and intermidateReceiver this an account at the middle chain
// that receives the token temporarily which then sends the token to receiver on destination chain via another ibc transfer packet.
// Note that the middle chain must support packet forward wrapper module (https://github.com/strangelove-ventures/packet-forward-middleware).
func (k Keeper) TransmitForwardIbcTransfer(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	transferPort, transferChannel string,
	token sdk.Coin,
	fwdTransferPort, fwdTransferChannel string,
	intermediateReceiver string,
	receiver string,
	transferTimeoutHeight ibcclienttypes.Height,
	transferTimeoutTimestamp uint64) error {
	fwdReceiver := buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)

	return k.TransmitIbcTransfer(
		ctx,
		owner,
		connectionId,
		timeoutTimestamp,
		transferPort, transferChannel,
		token,
		fwdReceiver,
		transferTimeoutHeight,
		transferTimeoutTimestamp,
	)
}

// buildPacketForwardReceiver builds the receiver address for packet forward transfer based on the format below:
// {intermediate_refund_address}|{foward_port}/{forward_channel}:{final_destination_address}
func buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver string) string {
	return fmt.Sprintf("%s|%s/%s:%s", intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)
}

func (k Keeper) sendTx(ctx sdk.Context, owner, connectionId string, msgs []sdk.Msg, timeoutTimestamp uint64) error {
	portID, err := icatypes.NewControllerPortID(owner)
	if err != nil {
		return err
	}
	channelID, found := k.icaControllerKeeper.GetActiveChannelID(ctx, connectionId, portID)
	if !found {
		return sdkerrors.Wrapf(icatypes.ErrActiveChannelNotFound, "failed to retrieve active channel for port %s", portID)
	}
	chanCap, found := k.scopedKeeper.GetCapability(ctx, host.ChannelCapabilityPath(portID, channelID))
	if !found {
		return sdkerrors.Wrap(channeltypes.ErrChannelCapabilityNotFound, "module does not own channel capability")
	}

	data, err := icatypes.SerializeCosmosTx(k.cdc, msgs)
	if err != nil {
		return err
	}

	packetData := icatypes.InterchainAccountPacketData{
		Type: icatypes.EXECUTE_TX,
		Data: data,
	}

	timeoutNano := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp
	seq, err := k.icaControllerKeeper.SendTx(ctx, chanCap, connectionId, portID, packetData, timeoutNano)
	if err != nil {
		return err
	}

	k.Logger(ctx).Info("sendTx ICA", "seq", seq)

	return nil
}

func (k Keeper) TransferIbcTokens(
	ctx sdk.Context,
	srcPort, srcChannel string,
	token sdk.Coin,
	sender sdk.AccAddress,
	receiver string,
	timeoutHeight ibcclienttypes.Height,
	timeoutTimestamp uint64,
) error {
	return k.ibcTransferKeeper.SendTransfer(
		ctx,
		srcPort,
		srcChannel,
		token,
		sender,
		receiver,
		timeoutHeight,
		timeoutTimestamp,
	)
}

func (k Keeper) ForwardTransferIbcTokens(
	ctx sdk.Context,
	srcPort, srcChannel string,
	token sdk.Coin,
	sender sdk.AccAddress,
	fwdTransferPort, fwdTransferChannel string,
	intermediateReceiver string,
	receiver string,
	timeoutHeight ibcclienttypes.Height,
	timeoutTimestamp uint64,
) error {
	fwdReceiver := buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)

	return k.ibcTransferKeeper.SendTransfer(
		ctx,
		srcPort,
		srcChannel,
		token,
		sender,
		fwdReceiver,
		timeoutHeight,
		timeoutTimestamp,
	)
}
