package suite

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"io/ioutil"
	"os"
	"strconv"
	"strings"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/relayer/rly"
	"github.com/strangelove-ventures/interchaintest/v4/testreporter"
)

type Primitive struct {
	PrimitiveAddress string
	CodeID           uint64
	Label            string
	InitMessage      string
}

type InitMessages struct {
	Init []string `json:"init_messages"`
}

func (p *Primitive) InstantiateContract(ctx context.Context, acc *ibc.Wallet, chain *cosmos.CosmosChain, funds sdk.Coins) error {
	if p.Label == "" || p.CodeID <= 0 {
		return fmt.Errorf("label or code ID is not correctly instantiated")
	}

	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(p.InitMessage)
	if err != nil {
		return err
	}
	cmds := []string{"wasm", "instantiate",
		strconv.FormatUint(p.CodeID, 10),
		string(argsbz),
		"--gas", "20000000",
	}
	if p.Label != "" {
		cmds = append(cmds, "--label", p.Label)
	}
	if acc.KeyName != "" {
		cmds = append(cmds, "--admin", acc.KeyName)
	} else {
		// We must set this explicitly if we don't want an admin
		cmds = append(cmds, "--no-admin")
	}
	if !funds.Empty() {
		cmds = append(cmds, "--amount", funds.String())
	}

	txhash, err := tn.ExecTx(ctx, acc.KeyName, cmds...)
	if err != nil {
		return fmt.Errorf(err.Error(), "failed to instantiate contract")
	}

	var resp wasmtypes.MsgInstantiateContractResponse

	txhashBytes, err := hex.DecodeString(txhash)
	if err != nil {
		return err
	}
	res, err := tn.Client.Tx(ctx, txhashBytes, false)
	if err != nil {
		return fmt.Errorf(err.Error(), "failed to find tx result %s", txhash)
	}
	if res.TxResult.Code != 0 {
		return fmt.Errorf("tx has non-zero code (%d) with log: %s", res.TxResult.Code, res.TxResult.Log)
	}

	// Only unmarshal result if user wants to
	if &resp != nil {
		err = unmarshalTxResult(res.TxResult.Data, &resp)
		return err
	}

	p.PrimitiveAddress = resp.Address
	return nil
}

func (p *Primitive) CreateICQChannel(ctx context.Context, relayer *rly.CosmosRelayer, erep *testreporter.RelayerExecReporter) error {
	return relayer.CreateChannel(
		ctx, erep, Quasar2OsmosisPath, ibc.CreateChannelOptions{
			SourcePortName: fmt.Sprintf("wasm.%s", p.PrimitiveAddress),
			DestPortName:   "icqhost",
			Order:          ibc.Unordered,
			Version:        "icq-1",
			Override:       true,
		})
}

func (p *Primitive) CreateICAChannel(ctx context.Context, relayer *rly.CosmosRelayer, erep *testreporter.RelayerExecReporter, connectionID, counterPartyConnectionID string) error {
	if p.PrimitiveAddress == "" {
		return fmt.Errorf("primitive not initialised")
	}
	return relayer.CreateChannel(
		ctx, erep, Quasar2OsmosisPath, ibc.CreateChannelOptions{
			SourcePortName: fmt.Sprintf("wasm.%s", p.PrimitiveAddress),
			DestPortName:   "icahost",
			Order:          ibc.Ordered,
			Version: fmt.Sprintf(
				`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
				connectionID,
				counterPartyConnectionID,
			),
			Override: true,
		})
}

func (p *Primitive) QueryPrimitive(ctx context.Context, chain *cosmos.CosmosChain, args any) ([]byte, error) {
	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(args)
	if err != nil {
		return nil, err
	}

	cmds := []string{"wasm", "contract-state", "smart",
		p.PrimitiveAddress,
		string(argsbz),
		"--output", "json",
	}

	res, _, err := tn.ExecQuery(ctx, cmds...)
	if err != nil {
		return nil, fmt.Errorf(err.Error(), "failed to execute query : "+strings.Join(cmds, " "))
	}

	return res, nil
}

func ReadInitMessagesFile(path string) ([]Primitive, error) {
	jsonFile, err := os.Open(path)
	// if we os.Open returns an error then handle it
	if err != nil {
		return nil, err
	}
	fmt.Printf("Successfully Opened %s", path)

	byteValue, err := ioutil.ReadAll(jsonFile)
	if err != nil {
		return nil, err
	}

	// we initialize our Users array
	var initMessages InitMessages

	// we unmarshal our byteArray which contains our
	// jsonFile's content into '' which we defined above
	err = json.Unmarshal(byteValue, &initMessages)
	if err != nil {
		return nil, err
	}

	var primitives []Primitive
	for _, im := range initMessages.Init {
		primitives = append(primitives, Primitive{
			InitMessage: im,
		})
	}

	return primitives, nil
}
