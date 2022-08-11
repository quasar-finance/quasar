package wasmbinding

import (
	"encoding/json"
	"errors"
	"fmt"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/CosmWasm/wasmd/x/wasm"
	wasmk "github.com/CosmWasm/wasmd/x/wasm/keeper"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	intergammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types"
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
		Sequence: seq,
	})
	if err != nil {
		return sdkerrors.Wrap(err, "ibc ack callback")
	}
	c.Logger(ctx).Info(fmt.Sprintf("Preparing callback message: %v", string(data)))

	// TODO make a type of this and send the sequence number together with this message
	msg := []byte(`{"ack_triggered": {}}`)

	res, err := c.contractKeeper.Execute(ctx, addr, addr, msg, nil)
	c.Logger(ctx).Debug(fmt.Sprintf("execute returned: %s", string(res)))

	return nil
}

type ContractAck struct {
	Sequence uint64 `json:"sequence_number"`
}

// OnSendPacket registers a packet's sequence number and address of the corresponding wasm contract
func (c *CallbackPlugin) OnSendPacket(ctx sdk.Context, seq uint64, addr sdk.AccAddress) {
	if c.sentMessages == nil {
		c.sentMessages = make(map[uint64]sdk.AccAddress)
	}
	c.sentMessages[seq] = addr
	// c.Logger(ctx).Info("callback address registered; seq: %d, addr: %v", seq, addr)
}
