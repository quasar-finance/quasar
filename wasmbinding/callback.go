package wasmbinding

import (
	"bytes"
	"encoding/json"
	"fmt"
	"strconv"

	"github.com/gogo/protobuf/proto"
	"github.com/gogo/protobuf/jsonpb"
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

func (c *CallbackPlugin) Handle(ctx sdk.Context, ex intergammtypes.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "handle")
}

func (c *CallbackPlugin) HandleAckMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "create_balancer_pool")
}

func (c *CallbackPlugin) HandleAckMsgJoinPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "join_pool")
}

func (c *CallbackPlugin) HandleAckMsgExitPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "exit_pool")
}

func (c *CallbackPlugin) HandleAckMsgJoinSwapExternAmountIn(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "join_swap_extern_amount_in")
}

func (c *CallbackPlugin) HandleAckMsgExitSwapExternAmountOut(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "exit_swap_extern_amount_out")
}

func (c *CallbackPlugin) HandleAckMsgJoinSwapShareAmountOut(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "join_swap_share_amount_out")
}

func (c *CallbackPlugin) HandleAckMsgExitSwapShareAmountIn(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "exit_swap_share_amount_in")
}

func (c *CallbackPlugin) HandleAckMsgLockTokens(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "lock_tokens")

}

func (c *CallbackPlugin) HandleAckMsgBeginUnlocking(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse],
) error {
	return c.doHandle(ctx, ex.Sequence, ex.Response, "begin_unlocking")
}

// the easiest way for the smart contract to handle the response is to 
func (c *CallbackPlugin) doHandle(ctx sdk.Context, seq uint64, response proto.Message, caller string) error {
	c.Logger(ctx).Error(fmt.Sprintf("trying to handle callback for sent message: %v", seq))
	addr, exists := c.sentMessages[seq]
	if !exists {
		// if the address does not exist, someone other than a smart contract called intergamm, thus we return nil.
		c.Logger(ctx).Error(fmt.Sprintf("wasm callback plugin called: no sent message found for: %v", seq))
		return nil
	}

	m := jsonpb.Marshaler{}
	resp := new(bytes.Buffer)
	m.Marshal(resp, response)

	data, err := json.Marshal(ContractAck{
		AckTriggered: struct {
			Sequence uint64 `json:"sequence_number"`
			Error string `json:"error,omitempty"`
			Response map[string]json.RawMessage `json:"response,omitempty"`
			}{
			Sequence: seq,
			Response: map[string]json.RawMessage{
				caller: resp.Bytes(),
			},
		},
	})

	if err != nil {
		return sdkerrors.Wrap(err, "ibc ack callback")
	}
	c.Logger(ctx).Info(fmt.Sprintf("Preparing callback message: %v", string(data)))

	// TODO hardcode the caller to the intergamm address
	res, err := c.contractKeeper.Execute(ctx, addr, addr, data, nil)
	c.Logger(ctx).Debug(fmt.Sprintf("execute returned: %s", string(res)))

	return nil
}

type ContractAck struct {
	AckTriggered struct {
		Sequence uint64 `json:"sequence_number"`
		Error string `json:"error,omitempty"`
		Response map[string]json.RawMessage `json:"response,omitempty"`
	} `json:"ack"`
}

// OnSendPacket registers a packet's sequence number and address of the corresponding wasm contract
func (c *CallbackPlugin) OnSendPacket(ctx sdk.Context, seq uint64, addr sdk.AccAddress) {
	if c.sentMessages == nil {
		c.sentMessages = make(map[uint64]sdk.AccAddress)
	}
	c.sentMessages[seq] = addr
	c.Logger(ctx).Info("Registering SEQ for contract addr", strconv.FormatUint(seq, 10), addr.String())
}
