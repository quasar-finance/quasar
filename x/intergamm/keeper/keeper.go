package keeper

import (
	"fmt"

	"github.com/tendermint/tendermint/libs/log"

	gammbalancer "github.com/abag/quasarnode/x/gamm/pool-models/balancer"
	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	icacontrollerkeeper "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/controller/keeper"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
)

type (
	Keeper struct {
		cdc                 codec.BinaryCodec
		storeKey            sdk.StoreKey
		memKey              sdk.StoreKey
		scopedKeeper        capabilitykeeper.ScopedKeeper
		icaControllerKeeper icacontrollerkeeper.Keeper
		paramstore          paramtypes.Subspace
	}
)

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey sdk.StoreKey,
	scopedKeeper capabilitykeeper.ScopedKeeper,
	iaKeeper icacontrollerkeeper.Keeper,
	ps paramtypes.Subspace,

) Keeper {
	// set KeyTable if it has not already been set
	if !ps.HasKeyTable() {
		ps = ps.WithKeyTable(types.ParamKeyTable())
	}

	return Keeper{
		cdc:                 cdc,
		storeKey:            storeKey,
		memKey:              memKey,
		scopedKeeper:        scopedKeeper,
		icaControllerKeeper: iaKeeper,
		paramstore:          ps,
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

// ClaimCapability claims the channel capability passed via the OnOpenChanInit callback
func (k *Keeper) ClaimCapability(ctx sdk.Context, cap *capabilitytypes.Capability, name string) error {
	return k.scopedKeeper.ClaimCapability(ctx, cap, name)
}

func (k Keeper) RegisterInterchainAccount(ctx sdk.Context, connectionID, owner string) error {
	return k.icaControllerKeeper.RegisterInterchainAccount(ctx, connectionID, owner)
}

func (k Keeper) TransmitIbcCreatePool(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	poolParams *gammbalancer.BalancerPoolParams,
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
	_, err = k.icaControllerKeeper.SendTx(ctx, chanCap, connectionId, portID, packetData, timeoutTimestamp)
	if err != nil {
		return err
	}

	return nil
}
