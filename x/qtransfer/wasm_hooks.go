package qtransfer

import (
	"encoding/json"
	"errors"
	"fmt"

	//	sdkerrors "cosmossdk.io/errors"
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	transfertypes "github.com/cosmos/ibc-go/v4/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	ibcexported "github.com/cosmos/ibc-go/v4/modules/core/exported"
	"github.com/quasarlabs/quasarnode/x/qtransfer/keeper"
	"github.com/quasarlabs/quasarnode/x/qtransfer/types"
)

type ContractAck struct {
	ContractResult []byte `json:"contract_result"`
	IbcAck         []byte `json:"ibc_ack"`
}

type WasmHooks struct {
	keeper             keeper.Keeper
	wasmKeeper         wasmkeeper.Keeper
	permissionedKeeper *wasmkeeper.PermissionedKeeper
}

func NewWasmHooks(k keeper.Keeper, wasmKeeper wasmkeeper.Keeper) WasmHooks {
	return WasmHooks{
		keeper:             k,
		wasmKeeper:         wasmKeeper,
		permissionedKeeper: wasmkeeper.NewDefaultPermissionKeeper(wasmKeeper),
	}
}

func (h WasmHooks) OnRecvPacketOverride(im IBCMiddleware, ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress) ibcexported.Acknowledgement {
	if !h.keeper.WasmHooksEnabled(ctx) {
		// Wasm hooks are disabled
		return im.App.OnRecvPacket(ctx, packet, relayer)
	}

	isIcs20, data := isIcs20Packet(packet)
	if !isIcs20 {
		return im.App.OnRecvPacket(ctx, packet, relayer)
	}

	// Validate the memo
	isWasmRouted, contractAddr, msgBytes, err := validateAndParseTransferMemo(data.GetMemo(), data.Receiver)
	if !isWasmRouted {
		return im.App.OnRecvPacket(ctx, packet, relayer)
	}
	if err != nil {
		return channeltypes.NewErrorAcknowledgement(err)
	}
	if msgBytes == nil || contractAddr == nil { // This should never happen
		return channeltypes.NewErrorAcknowledgement(errors.New("error in wasmhook message validation"))
	}

	// The funds sent on this packet need to be transferred to the wasm hooks module address/
	// For this, we override the ICS20 packet's Receiver (essentially hijacking the funds for the module)
	// and execute the underlying OnRecvPacket() call (which should eventually land on the transfer app's
	// relay.go and send the funds to the module.
	//
	// If that succeeds, we make the contract call
	data.Receiver = types.IntermediateAccountAddress.String()
	bz, err := json.Marshal(data)
	if err != nil {
		return channeltypes.NewErrorAcknowledgement(fmt.Errorf("cannot marshal the ICS20 packet: %w", err))
	}
	packet.Data = bz

	// Execute the receive
	ack := im.App.OnRecvPacket(ctx, packet, relayer)
	if !ack.Success() {
		return ack
	}

	amount, ok := sdk.NewIntFromString(data.GetAmount())
	if !ok {
		// This should never happen, as it should've been caught in the underlying call to OnRecvPacket,
		// but returning here for completeness
		return channeltypes.NewErrorAcknowledgement(fmt.Errorf("invalid packet data: Amount is not an int"))
	}

	// The packet's denom is the denom in the sender chain. This needs to be converted to the local denom.
	denom := MustExtractDenomFromPacketOnRecv(packet)
	funds := sdk.NewCoins(sdk.NewCoin(denom, amount))

	execMsg := wasmtypes.MsgExecuteContract{
		Sender:   types.IntermediateAccountAddress.String(),
		Contract: contractAddr.String(),
		Msg:      msgBytes,
		Funds:    funds,
	}
	response, err := h.execWasmMsg(ctx, &execMsg)
	if err != nil {
		return channeltypes.NewErrorAcknowledgement(err)
	}

	fullAck := ContractAck{ContractResult: response.Data, IbcAck: ack.Acknowledgement()}
	bz, err = json.Marshal(fullAck)
	if err != nil {
		return channeltypes.NewErrorAcknowledgement(sdkerrors.Wrap(err, "cannot marshal the contract acknowledgement"))
	}

	return channeltypes.NewResultAcknowledgement(bz)
}

// MustExtractDenomFromPacketOnRecv takes a packet with a valid ICS20 token data in the Data field and returns the
// denom as represented in the local chain.
// If the data cannot be unmarshalled this function will panic
func MustExtractDenomFromPacketOnRecv(packet ibcexported.PacketI) string {
	var data transfertypes.FungibleTokenPacketData
	if err := json.Unmarshal(packet.GetData(), &data); err != nil {
		panic("unable to unmarshal ICS20 packet data")
	}

	var denom string
	if transfertypes.ReceiverChainIsSource(packet.GetSourcePort(), packet.GetSourceChannel(), data.Denom) {
		// remove prefix added by sender chain
		voucherPrefix := transfertypes.GetDenomPrefix(packet.GetSourcePort(), packet.GetSourceChannel())

		unprefixedDenom := data.Denom[len(voucherPrefix):]

		// coin denomination used in sending from the escrow address
		denom = unprefixedDenom

		// The denomination used to send the coins is either the native denom or the hash of the path
		// if the denomination is not native.
		denomTrace := transfertypes.ParseDenomTrace(unprefixedDenom)
		if denomTrace.Path != "" {
			denom = denomTrace.IBCDenom()
		}
	} else {
		prefixedDenom := transfertypes.GetDenomPrefix(packet.GetDestPort(), packet.GetDestChannel()) + data.Denom
		denom = transfertypes.ParseDenomTrace(prefixedDenom).IBCDenom()
	}
	return denom
}

func (h WasmHooks) execWasmMsg(ctx sdk.Context, execMsg *wasmtypes.MsgExecuteContract) (*wasmtypes.MsgExecuteContractResponse, error) {
	if err := execMsg.ValidateBasic(); err != nil {
		return nil, sdkerrors.Wrap(err, "invalid wasm contract execution message")
	}
	wasmMsgServer := wasmkeeper.NewMsgServerImpl(h.permissionedKeeper)
	return wasmMsgServer.ExecuteContract(sdk.WrapSDKContext(ctx), execMsg)
}

func isIcs20Packet(packet channeltypes.Packet) (isIcs20 bool, ics20data transfertypes.FungibleTokenPacketData) {
	var data transfertypes.FungibleTokenPacketData
	if err := json.Unmarshal(packet.GetData(), &data); err != nil {
		return false, data
	}
	return true, data
}

func isMemoWasmRouted(memo string) (isWasmRouted bool, metadata map[string]interface{}) {
	metadata = make(map[string]interface{})

	// If there is no memo, the packet was either sent with an earlier version of IBC, or the memo was
	// intentionally left blank. Nothing to do here. Ignore the packet and pass it down the stack.
	if len(memo) == 0 {
		return false, metadata
	}

	// the metadata must be a valid JSON object
	err := json.Unmarshal([]byte(memo), &metadata)
	if err != nil {
		return false, metadata
	}

	// If the key "wasm" doesn't exist, there's nothing to do on this hook. Continue by passing the packet
	// down the stack
	_, ok := metadata["wasm"]
	if !ok {
		return false, metadata
	}

	return true, metadata
}

func validateAndParseTransferMemo(memo string, receiver string) (isWasmRouted bool, contractAddr sdk.AccAddress, msgBytes []byte, err error) {
	isWasmRouted, metadata := isMemoWasmRouted(memo)
	if !isWasmRouted {
		return isWasmRouted, sdk.AccAddress{}, nil, nil
	}

	wasmRaw := metadata["wasm"]

	// Make sure the wasm key is a map. If it isn't, ignore this packet
	wasm, ok := wasmRaw.(map[string]interface{})
	if !ok {
		return isWasmRouted, sdk.AccAddress{}, nil,
			sdkerrors.Wrap(types.ErrInvalidMetadataFormat, "wasm metadata is not a JSON map object")
	}

	// Get the contract
	contract, ok := wasm["contract"].(string)
	if !ok {
		// The tokens will be returned
		return isWasmRouted, sdk.AccAddress{}, nil,
			sdkerrors.Wrapf(types.ErrInvalidMetadataFormat, `could not find key wasm["contract"]`)
	}

	contractAddr, err = sdk.AccAddressFromBech32(contract)
	if err != nil {
		return isWasmRouted, sdk.AccAddress{}, nil,
			sdkerrors.Wrap(types.ErrInvalidMetadataFormat, `wasm["contract"] is not a valid bech32 address`)
	}

	// The contract and the receiver should be the same for the packet to be valid
	if contract != receiver {
		return isWasmRouted, sdk.AccAddress{}, nil,
			sdkerrors.Wrap(types.ErrInvalidMetadataFormat, `wasm["contract"] should be the same as the receiver of the packet`)
	}

	// Ensure the message key is provided
	if wasm["msg"] == nil {
		return isWasmRouted, sdk.AccAddress{}, nil,
			sdkerrors.Wrap(types.ErrInvalidMetadataFormat, `could not find key wasm["msg"]`)
	}

	// Make sure the msg key is a map. If it isn't, return an error
	_, ok = wasm["msg"].(map[string]interface{})
	if !ok {
		return isWasmRouted, sdk.AccAddress{}, nil,
			sdkerrors.Wrap(types.ErrInvalidMetadataFormat, `wasm["msg"] is not a map object`)
	}

	// Get the message string by serializing the map
	msgBytes, err = json.Marshal(wasm["msg"])
	if err != nil {
		// The tokens will be returned
		return isWasmRouted, sdk.AccAddress{}, nil,
			sdkerrors.Wrapf(types.ErrInvalidMetadataFormat, `could not marshal wasm["msg"] field back to json: %s`, err)
	}

	return isWasmRouted, contractAddr, msgBytes, nil
}

func (h WasmHooks) OnAcknowledgementPacketOverride(im IBCMiddleware, ctx sdk.Context, packet channeltypes.Packet, acknowledgement []byte, relayer sdk.AccAddress) error {
	err := im.App.OnAcknowledgementPacket(ctx, packet, acknowledgement, relayer)
	if err != nil {
		return err
	}

	transferPacket, err := unmarshalTransferPacket(packet)
	if err != nil {
		return err
	}
	ack, err := unmarshalAcknowledgement(acknowledgement)
	if err != nil {
		return err
	}

	contractAddr, err := sdk.AccAddressFromBech32(transferPacket.GetSender())
	if err != nil {
		return sdkerrors.Wrap(err, "failed to decode transfer packet sender address")
	}
	contractInfo := h.wasmKeeper.GetContractInfo(ctx, contractAddr)

	if contractInfo == nil {
		h.keeper.Logger(ctx).Info("ContractInfo not found for ",
			"address", contractAddr)
		return nil
	}

	if contractInfo.IBCPortID == "" {
		h.keeper.Logger(ctx).Info("Contract does not support ibc ",
			"address", contractAddr,
			"contractInfo", contractInfo.String())
		return nil
	}

	if !ack.Success() {
		h.keeper.Logger(ctx).Debug(
			"passing an error acknowledgment to contract",
			"contract_address", contractAddr,
			"error", ack.GetError(),
		)
	}
	err = h.wasmKeeper.OnAckPacket(ctx, contractAddr, wasmvmtypes.IBCPacketAckMsg{
		Acknowledgement: wasmvmtypes.IBCAcknowledgement{Data: acknowledgement},
		OriginalPacket:  newWasmIBCPacket(packet),
		Relayer:         relayer.String(),
	})

	if err != nil {
		h.keeper.Logger(ctx).Error(
			"contract returned error for acknowledgment",
			"contract_address", contractAddr,
			"error", err,
		)
		return sdkerrors.Wrap(err, "contract returned error for acknowledgment")
	}

	return nil
}

func unmarshalTransferPacket(packet channeltypes.Packet) (transfertypes.FungibleTokenPacketData, error) {
	var transferPacket transfertypes.FungibleTokenPacketData
	err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &transferPacket)
	if err != nil {
		return transferPacket, sdkerrors.Wrap(err, "cannot unmarshal ICS-20 transfer packet data")
	}

	return transferPacket, nil
}

func unmarshalAcknowledgement(acknowledgement []byte) (channeltypes.Acknowledgement, error) {
	var ack channeltypes.Acknowledgement
	err := types.ModuleCdc.UnmarshalJSON(acknowledgement, &ack)
	if err != nil {
		return ack, sdkerrors.Wrap(err, "cannot unmarshal ICS-20 transfer packet acknowledgement")
	}
	return ack, nil
}

func newWasmIBCPacket(packet channeltypes.Packet) wasmvmtypes.IBCPacket {
	timeout := wasmvmtypes.IBCTimeout{
		Timestamp: packet.TimeoutTimestamp,
	}
	if !packet.TimeoutHeight.IsZero() {
		timeout.Block = &wasmvmtypes.IBCTimeoutBlock{
			Height:   packet.TimeoutHeight.RevisionHeight,
			Revision: packet.TimeoutHeight.RevisionNumber,
		}
	}

	return wasmvmtypes.IBCPacket{
		Data:     packet.Data,
		Src:      wasmvmtypes.IBCEndpoint{ChannelID: packet.SourceChannel, PortID: packet.SourcePort},
		Dest:     wasmvmtypes.IBCEndpoint{ChannelID: packet.DestinationChannel, PortID: packet.DestinationPort},
		Sequence: packet.Sequence,
		Timeout:  timeout,
	}
}

func (h WasmHooks) OnTimeoutPacketOverride(im IBCMiddleware, ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress) error {
	err := im.App.OnTimeoutPacket(ctx, packet, relayer)
	if err != nil {
		return err
	}

	transferPacket, err := unmarshalTransferPacket(packet)
	if err != nil {
		return err
	}

	contractAddr, err := sdk.AccAddressFromBech32(transferPacket.GetSender())
	if err != nil {
		return sdkerrors.Wrap(err, "failed to decode transfer packet sender address")
	}
	contractInfo := h.wasmKeeper.GetContractInfo(ctx, contractAddr)
	// Skip if there's no contract with this address (it's a regular address) or the contract doesn't support IBC
	if contractInfo == nil || contractInfo.IBCPortID == "" {
		return nil
	}

	err = h.wasmKeeper.OnTimeoutPacket(ctx, contractAddr, wasmvmtypes.IBCPacketTimeoutMsg{
		Packet:  newWasmIBCPacket(packet),
		Relayer: relayer.String(),
	})
	if err != nil {
		h.keeper.Logger(ctx).Error(
			"contract returned error for timeout",
			"contract_address", contractAddr,
			"error", err,
		)
		return sdkerrors.Wrap(err, "contract returned error for timeout")
	}

	return nil
}
