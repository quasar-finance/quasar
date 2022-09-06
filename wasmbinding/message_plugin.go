package wasmbinding

import (
	"encoding/json"
	"time"

	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"

	"github.com/quasarlabs/quasarnode/wasmbinding/bindings"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	intergammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func CustomMessageDecorator(intergammKeeper *intergammkeeper.Keeper, bank *bankkeeper.BaseKeeper, callback *CallbackPlugin) func(wasmkeeper.Messenger) wasmkeeper.Messenger {
	return func(old wasmkeeper.Messenger) wasmkeeper.Messenger {
		return &CustomMessenger{
			wrapped:         old,
			bank:            bank,
			intergammKeeper: intergammKeeper,
			callback:        callback,
		}
	}
}

type CustomMessenger struct {
	wrapped         wasmkeeper.Messenger
	bank            *bankkeeper.BaseKeeper
	intergammKeeper *intergammkeeper.Keeper
	callback        *CallbackPlugin
}

var _ wasmkeeper.Messenger = (*CustomMessenger)(nil)

func (m *CustomMessenger) DispatchMsg(ctx sdk.Context, contractAddr sdk.AccAddress, contractIBCPortID string, msg wasmvmtypes.CosmosMsg) ([]sdk.Event, [][]byte, error) {
	if msg.Custom != nil {
		// only handle the happy path where this is really creating / minting / swapping ...
		// leave everything else for the wrapped version
		var contractMsg bindings.QuasarMsg
		if err := json.Unmarshal(msg.Custom, &contractMsg); err != nil {
			return nil, nil, sdkerrors.Wrap(err, "osmosis msg")
		}
		if contractMsg.TestScenario != nil {
			return m.testScenario(ctx, contractAddr, contractMsg.TestScenario)
		}
		if contractMsg.SendToken != nil {
			return m.sendToken(ctx, contractAddr, contractMsg.SendToken)
		}
		if contractMsg.RegisterInterchainAccount != nil {
			return m.RegisterInterchainAccount(ctx, contractAddr, contractMsg.RegisterInterchainAccount)
		}
		if contractMsg.OsmosisJoinPool != nil {
			return m.OsmosisJoinPool(ctx, contractAddr, contractMsg.OsmosisJoinPool)
		}
		if contractMsg.OsmosisExitPool != nil {
			return m.OsmosisExitPool(ctx, contractAddr, contractMsg.OsmosisExitPool)
		}
		if contractMsg.OsmosisLockTokens != nil {
			return m.OsmosisLockTokens(ctx, contractAddr, contractMsg.OsmosisLockTokens)
		}
		if contractMsg.OsmosisBeginUnlocking != nil {
			return m.OsmosisBeginUnlocking(ctx, contractAddr, contractMsg.OsmosisBeginUnlocking)
		}
		if contractMsg.OsmosisJoinSwapExternAmountIn != nil {
			return m.OsmosisJoinSwapExternAmountIn(ctx, contractAddr, contractMsg.OsmosisJoinSwapExternAmountIn)
		}
		if contractMsg.OsmosisExitSwapExternAmountOut != nil {
			return m.OsmosisExitSwapExternAmountOut(ctx, contractAddr, contractMsg.OsmosisExitSwapExternAmountOut)
		}
	}
	return m.wrapped.DispatchMsg(ctx, contractAddr, contractIBCPortID, msg)
}

func (m *CustomMessenger) testScenario(ctx sdk.Context, contractAddr sdk.AccAddress, testScenario *bindings.TestScenario) ([]sdk.Event, [][]byte, error) {
	err := PerformTestScenario(m.intergammKeeper, ctx, contractAddr, testScenario)
	// err := PerformCreateDenom(m.tokenFactory, m.bank, ctx, contractAddr, createDenom)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "perform test scenario")
	}
	return nil, nil, nil
}

func PerformTestScenario(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.AccAddress, testScenario *bindings.TestScenario) error {
	if testScenario == nil {
		return wasmvmtypes.InvalidRequest{Err: "test scenario null"}
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)

	msgTestScenario := intergammtypes.NewMsgTestScenario(contractAddr.String(), testScenario.Scenario)

	// msgCreateDenom := tokenfactorytypes.NewMsgCreateDenom(contractAddr.String(), createDenom.Subdenom)

	if err := msgTestScenario.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "failed validating MsgTestScenario")
	}

	// Run the test scenario
	_, err := msgServer.TestScenario(
		sdk.WrapSDKContext(ctx),
		msgTestScenario,
	)
	if err != nil {
		return sdkerrors.Wrap(err, "running test scenario")
	}
	return nil
}

func (m *CustomMessenger) sendToken(ctx sdk.Context, contractAddr sdk.AccAddress, send *bindings.SendToken) ([]sdk.Event, [][]byte, error) {
	err := PerformSendToken(m.intergammKeeper, m.bank, ctx, contractAddr, send, m.callback)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "send token")
	}
	return nil, nil, nil
}

func PerformSendToken(k *intergammkeeper.Keeper, b *bankkeeper.BaseKeeper, ctx sdk.Context, contractAddr sdk.AccAddress, send *bindings.SendToken, cb *CallbackPlugin) error {
	if send == nil {
		return wasmvmtypes.InvalidRequest{Err: "send token null"}
	}
	receiver, err := parseAddress(send.Receiver) // where to use?
	if err != nil {
		return sdkerrors.Wrap(err, "parse receiver")
	}

	sdkMsg := intergammtypes.NewMsgSendToken(send.Creator, send.DestinationLocalZoneId, send.Sender, receiver.String(), &send.Coin)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	res, err := msgServer.SendToken(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "send token")
	}

	// register the packet as sent with the callback plugin
	cb.OnSendPacket(ctx, res.GetSeq(), contractAddr)

	if err != nil {
		return sdkerrors.Wrap(err, "sending tokens")
	}
	return nil
}

func (m *CustomMessenger) RegisterInterchainAccount(ctx sdk.Context, contractAddr sdk.Address, register *bindings.RegisterInterchainAccount) ([]sdk.Event, [][]byte, error) {
	err := PerformRegisterInterchainAccount(m.intergammKeeper, ctx, contractAddr, register)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "register ica account")
	}
	return nil, nil, nil
}

func PerformRegisterInterchainAccount(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.Address, register *bindings.RegisterInterchainAccount) error {
	if register == nil {
		return wasmvmtypes.InvalidRequest{Err: "register interchain account null"}
	}

	sdkMsg := intergammtypes.NewMsgRegisterInterchainAccount(contractAddr.String(), register.ConnectionId)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	_, err := msgServer.RegisterInterchainAccount(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "register interchain account")
	}
	return nil
}

func (m *CustomMessenger) OsmosisJoinPool(ctx sdk.Context, contractAddr sdk.AccAddress, join *bindings.OsmosisJoinPool) ([]sdk.Event, [][]byte, error) {
	err := PerformOsmosisJoinPool(m.intergammKeeper, ctx, contractAddr, join, m.callback)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "join pool")
	}
	return nil, nil, nil
}

func PerformOsmosisJoinPool(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.AccAddress, join *bindings.OsmosisJoinPool, cb *CallbackPlugin) error {
	if join == nil {
		return wasmvmtypes.InvalidRequest{Err: "join pool null"}
	}

	sdkMsg := intergammtypes.NewMsgTransmitIbcJoinPool(join.Creator, join.ConnectionId, join.TimeoutTimestamp, join.PoolId, join.ShareOutAmount, join.TokenInMaxs)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	res, err := msgServer.TransmitIbcJoinPool(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "join pool")
	}

	cb.OnSendPacket(ctx, res.Seq, contractAddr)
	return nil
}

func (m *CustomMessenger) OsmosisExitPool(ctx sdk.Context, contractAddr sdk.AccAddress, exit *bindings.OsmosisExitPool) ([]sdk.Event, [][]byte, error) {
	err := PerformOsmosisExitPool(m.intergammKeeper, ctx, contractAddr, exit, m.callback)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "exit pool")
	}
	return nil, nil, nil
}

func PerformOsmosisExitPool(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.AccAddress, exit *bindings.OsmosisExitPool, cb *CallbackPlugin) error {
	if exit == nil {
		return wasmvmtypes.InvalidRequest{Err: "exit pool null"}
	}

	sdkMsg := intergammtypes.NewMsgTransmitIbcExitPool(exit.Creator, exit.ConnectionId, exit.TimeoutTimestamp, exit.PoolId, exit.ShareInAmount, exit.TokenOutMins)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	res, err := msgServer.TransmitIbcExitPool(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "exit pool")
	}

	cb.OnSendPacket(ctx, res.GetSeq(), contractAddr)
	return nil
}

func (m *CustomMessenger) OsmosisLockTokens(ctx sdk.Context, contractAddr sdk.AccAddress, withdraw *bindings.OsmosisLockTokens) ([]sdk.Event, [][]byte, error) {
	err := PerformOsmosisLockTokens(m.intergammKeeper, ctx, contractAddr, withdraw, m.callback)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "withdraw")
	}
	return nil, nil, nil
}

func PerformOsmosisLockTokens(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.AccAddress, lock *bindings.OsmosisLockTokens, cb *CallbackPlugin) error {
	if lock == nil {
		return wasmvmtypes.InvalidRequest{Err: "withdraw null"}
	}

	// TODO: lets make sure the way we do durations is correct
	sdkMsg := intergammtypes.NewMsgTransmitIbcLockTokens(lock.Creator, lock.ConnectionId, lock.TimeoutTimestamp, time.Duration(lock.Duration), lock.Coins)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	res, err := msgServer.TransmitIbcLockTokens(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "lock tokens")
	}

	cb.OnSendPacket(ctx, res.GetSeq(), contractAddr)
	return nil
}

func (m *CustomMessenger) OsmosisBeginUnlocking(ctx sdk.Context, contractAddr sdk.AccAddress, begin *bindings.OsmosisBeginUnlocking) ([]sdk.Event, [][]byte, error) {
	err := PerformOsmosisBeginUnlocking(m.intergammKeeper, ctx, contractAddr, begin, m.callback)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "begin unlocking")
	}
	return nil, nil, nil
}

func PerformOsmosisBeginUnlocking(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.AccAddress, begin *bindings.OsmosisBeginUnlocking, cb *CallbackPlugin) error {
	if begin == nil {
		return wasmvmtypes.InvalidRequest{Err: "begin unlocking null"}
	}

	sdkMsg := intergammtypes.NewMsgTransmitIbcBeginUnlocking(begin.Creator, begin.ConnectionId, begin.TimeoutTimestamp, begin.Id, begin.Coins)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	res, err := msgServer.TransmitIbcBeginUnlocking(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "begin unlocking")
	}

	cb.OnSendPacket(ctx, res.GetSeq(), contractAddr)
	return nil
}

func (m *CustomMessenger) OsmosisJoinSwapExternAmountIn(ctx sdk.Context, contractAddr sdk.AccAddress, join *bindings.OsmosisJoinSwapExternAmountIn) ([]sdk.Event, [][]byte, error) {
	err := PerformOsmosisJoinSwapExternAmountIn(m.intergammKeeper, ctx, contractAddr, join, m.callback)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "join swap extern amount in")
	}
	return nil, nil, nil
}

func PerformOsmosisJoinSwapExternAmountIn(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.AccAddress, join *bindings.OsmosisJoinSwapExternAmountIn, cb *CallbackPlugin) error {
	if join == nil {
		return wasmvmtypes.InvalidRequest{Err: "join swap extern amount in null"}
	}

	sdkMsg := intergammtypes.NewMsgTransmitIbcJoinSwapExternAmountIn(join.Creator, join.ConnectionId, join.TimeoutTimestamp, join.PoolId, join.ShareOutMinAmount, join.TokenIn)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	res, err := msgServer.TransmitIbcJoinSwapExternAmountIn(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "join swap extern amount in")
	}

	cb.OnSendPacket(ctx, res.GetSeq(), contractAddr)
	return nil
}

func (m *CustomMessenger) OsmosisExitSwapExternAmountOut(ctx sdk.Context, contractAddr sdk.AccAddress, exit *bindings.OsmosisExitSwapExternAmountOut) ([]sdk.Event, [][]byte, error) {
	err := PerformOsmosisExitSwapExternAmountOut(m.intergammKeeper, ctx, contractAddr, exit, m.callback)
	if err != nil {
		return nil, nil, sdkerrors.Wrap(err, "exit swap extern amount out")
	}
	return nil, nil, nil
}

func PerformOsmosisExitSwapExternAmountOut(k *intergammkeeper.Keeper, ctx sdk.Context, contractAddr sdk.AccAddress, exit *bindings.OsmosisExitSwapExternAmountOut, cb *CallbackPlugin) error {
	if exit == nil {
		return wasmvmtypes.InvalidRequest{Err: "exit swap extern amount out null"}
	}

	sdkMsg := intergammtypes.NewMsgTransmitIbcExitSwapExternAmountOut(exit.Creator, exit.ConnectionId, exit.TimeoutTimestamp, exit.PoolId, exit.ShareInAmount, exit.TokenOutMins)
	if err := sdkMsg.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "basic validate msg")
	}

	msgServer := intergammkeeper.NewMsgServerImpl(k)
	res, err := msgServer.TransmitIbcExitSwapExternAmountOut(sdk.WrapSDKContext(ctx), sdkMsg)
	if err != nil {
		return sdkerrors.Wrap(err, "join swap extern amount out")
	}

	cb.OnSendPacket(ctx, res.GetSeq(), contractAddr)
	return nil
}

func parseAddress(addr string) (sdk.AccAddress, error) {
	parsed, err := sdk.AccAddressFromBech32(addr)
	if err != nil {
		return nil, sdkerrors.Wrap(err, "address from bech32")
	}
	err = sdk.VerifyAddressFormat(parsed)
	if err != nil {
		return nil, sdkerrors.Wrap(err, "verify address format")
	}
	return parsed, nil
}
