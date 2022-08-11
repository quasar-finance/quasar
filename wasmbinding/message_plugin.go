package wasmbinding

import (
	"encoding/json"

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
	_, err = msgServer.SendToken(sdk.WrapSDKContext(ctx), sdkMsg)

	// hardcode seq for POC
	// register the packet as sent with the callback plugin
	cb.OnSendPacket(ctx, 1, contractAddr)
	// for testing, trigger the callback in the contract
	cb.doHandle(ctx, 1)

	// TODO stop ignoring the error once we have a test setup and trigger the contract on the Handle of the ack
	// ignore the error for now
	// if err != nil {
	// 	return sdkerrors.Wrap(err, "sending tokens")
	// }
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
