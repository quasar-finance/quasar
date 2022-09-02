package wasmbinding

import (
	"encoding/json"
	"errors"
	"fmt"
	"strconv"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/CosmWasm/wasmd/x/wasm"
	wasmk "github.com/CosmWasm/wasmd/x/wasm/keeper"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	intergammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types"
	gammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types/osmosis/v9/gamm"
	gammbalancer "github.com/quasarlabs/quasarnode/x/intergamm/types/osmosis/v9/gamm/pool-models/balancer"
	lockuptypes "github.com/quasarlabs/quasarnode/x/intergamm/types/osmosis/v9/lockup"
)

// if we want to use this plugin to also call the execute entrypoint, we also need to give the ContractOpsKeeper(https://github.com/CosmWasm/wasmd/blob/main/x/wasm/types/exported_keepers.go)
func NewCallbackPlugin(k *wasm.Keeper) *CallbackPlugin {
	return &CallbackPlugin{
		wasmkeeper:     k,
		sentMessages:   map[uint64]sdk.AccAddress{},
		contractKeeper: wasmk.NewDefaultPermissionKeeper(k),
	}
}

type CallbackPlugin struct {
	// TODO evaluate whether we need wasm keeper in the callback plugin
	wasmkeeper     *wasm.Keeper
	contractKeeper *wasmk.PermissionedKeeper
	sentMessages   map[uint64]sdk.AccAddress
}

func (c *CallbackPlugin) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("wasm callback plugin")
}

func (c *CallbackPlugin) Handle(ctx sdk.Context, ack intergammtypes.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]) error {
	return c.doHandle(ctx, ack.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgJoinPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgExitPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgJoinSwapExternAmountIn(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgExitSwapExternAmountOut(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgJoinSwapShareAmountOut(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgExitSwapShareAmountIn(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

func (c *CallbackPlugin) HandleAckMsgLockTokens(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)

}

func (c *CallbackPlugin) HandleAckMsgBeginUnlocking(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse],
) error {
	return c.doHandle(ctx, ex.Sequence)
}

// TODO add request and response in here
func (c *CallbackPlugin) doHandle(ctx sdk.Context, seq uint64) error {
	c.Logger(ctx).Error(fmt.Sprintf("trying to handle callback for sent message: %v", seq))
	addr, exists := c.sentMessages[seq]
	if !exists {
		// intergamm hooks require there to be no error, thus we return nil here
		c.Logger(ctx).Error(fmt.Sprintf("wasm callback plugin called: no sent message found for: %v", seq))
		return errors.New("sequence number and address not found")
	}

	data, err := json.Marshal(ContractAck{
		AckTriggered: struct {
			Sequence uint64 "json:\"sequence_number\""
		}{
			Sequence: seq,
		},
	})

	if err != nil {
		return sdkerrors.Wrap(err, "ibc ack callback")
	}
	c.Logger(ctx).Info(fmt.Sprintf("Preparing callback message: %v", string(data)))

	res, err := c.contractKeeper.Execute(ctx, addr, addr, data, nil)
	c.Logger(ctx).Debug(fmt.Sprintf("execute returned: %s", string(res)))

	return nil
}

type ContractAck struct {
	AckTriggered struct {
		Sequence uint64 `json:"sequence_number"`
	} `json:"ack_triggered"`
}

// OnSendPacket registers a packet's sequence number and address of the corresponding wasm contract
func (c *CallbackPlugin) OnSendPacket(ctx sdk.Context, seq uint64, addr sdk.AccAddress) {
	if c.sentMessages == nil {
		c.sentMessages = make(map[uint64]sdk.AccAddress)
	}
	c.sentMessages[seq] = addr
	c.Logger(ctx).Info("Registering SEQ for contract addr", strconv.FormatUint(seq, 10), addr.String())
}
