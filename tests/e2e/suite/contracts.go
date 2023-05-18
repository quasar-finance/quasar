package suite

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/relayer/rly"
	"github.com/strangelove-ventures/interchaintest/v4/testreporter"
	"go.uber.org/zap"

	"github.com/quasarlabs/quasarnode/tests/e2e/dockerutil"
)

type Contract struct {
	contractType    string
	contractAddress string
	codeID          uint64
	label           string
	initMessage     any
}

type ContractDetails struct {
	ContractDetails []ContractDetail `json:"contract_details"`
}

type ContractDetail struct {
	InitMessage  any    `json:"init_message"`
	Label        string `json:"label"`
	ContractType string `json:"contract_type"`
}

// NewContract returns Contract struct with init message assigned to it if codeID is zero
func NewContract(initMsg any, label string, codeID uint64) *Contract {
	if codeID == 0 {
		return &Contract{
			initMessage: initMsg,
			label:       label,
		}
	} else {
		return &Contract{
			codeID:      codeID,
			initMessage: initMsg,
			label:       label,
		}
	}
}

func (p *Contract) SetCodeID(codeID uint64) {
	p.codeID = codeID
}

func (p *Contract) GetCodeID() uint64 {
	return p.codeID
}

func (p *Contract) GetContractAddress() string {
	return p.contractAddress
}

func (p *Contract) InstantiateContract(ctx context.Context, acc *ibc.Wallet, chain *cosmos.CosmosChain, funds sdk.Coins) error {
	if p.label == "" || p.codeID <= 0 {
		return fmt.Errorf("label or code ID is not correctly instantiated")
	}

	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(p.initMessage)
	if err != nil {
		return err
	}
	cmds := []string{"wasm", "instantiate",
		strconv.FormatUint(p.codeID, 10),
		string(argsbz),
		"--gas", "20000000",
	}
	if p.label != "" {
		cmds = append(cmds, "--label", p.label)
	}

	accAddress := acc.Bech32Address(chain.Config().Bech32Prefix)
	if acc.KeyName != "" {
		cmds = append(cmds, "--admin", accAddress)
	} else {
		// We must set this explicitly if we don't want an admin
		cmds = append(cmds, "--no-admin")
	}
	if !funds.Empty() {
		cmds = append(cmds, "--amount", funds.String())
	}

	txhash, err := tn.ExecTx(ctx, acc.KeyName, cmds...)
	if err != nil {
		return fmt.Errorf(err.Error(), "failed to instantiate Contract")
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
		if err != nil {
			return err
		}
	}

	p.contractAddress = resp.Address
	return nil
}

func (p *Contract) CreateICQChannel(ctx context.Context, relayer *rly.CosmosRelayer, erep *testreporter.RelayerExecReporter) error {
	if p.label == "" || p.codeID <= 0 || p.contractAddress == "" {
		return fmt.Errorf("label, code ID or Contract address is not correctly instantiated")
	}
	return relayer.CreateChannel(
		ctx, erep, Quasar2OsmosisPath, ibc.CreateChannelOptions{
			SourcePortName: fmt.Sprintf("wasm.%s", p.contractAddress),
			DestPortName:   "icqhost",
			Order:          ibc.Unordered,
			Version:        "icq-1",
			Override:       true,
		})
}

func (p *Contract) CreateICAChannel(ctx context.Context, relayer *rly.CosmosRelayer, erep *testreporter.RelayerExecReporter, connectionID, counterPartyConnectionID string) error {
	if p.label == "" || p.codeID <= 0 || p.contractAddress == "" {
		return fmt.Errorf("label, code ID or Contract address is not correctly instantiated")
	}
	return relayer.CreateChannel(
		ctx, erep, Quasar2OsmosisPath, ibc.CreateChannelOptions{
			SourcePortName: fmt.Sprintf("wasm.%s", p.contractAddress),
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

func (p *Contract) QueryContract(ctx context.Context, chain *cosmos.CosmosChain, args any) ([]byte, error) {
	if p.contractAddress == "" {
		return nil, fmt.Errorf("primitive not initialised")
	}

	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(args)
	if err != nil {
		return nil, err
	}

	cmds := []string{"wasm", "Contract-state", "smart",
		p.contractAddress,
		string(argsbz),
		"--output", "json",
	}

	res, _, err := tn.ExecQuery(ctx, cmds...)
	if err != nil {
		return nil, fmt.Errorf(err.Error(), "failed to execute query : "+strings.Join(cmds, " "))
	}

	return res, nil
}

func (p *Contract) ExecuteContract(ctx context.Context, chain *cosmos.CosmosChain, args, result any, funds sdk.Coins, acc *ibc.Wallet) (any, error) {
	if p.contractAddress == "" {
		return nil, fmt.Errorf("primitive not initialised")
	}

	tn := GetFullNode(chain)

	argsbz, err := json.Marshal(args)
	if err != nil {
		return nil, err
	}

	cmds := []string{"wasm", "execute",
		p.contractAddress,
		string(argsbz),
		"--gas", "20000000",
	}
	if !funds.Empty() {
		cmds = append(cmds, "--amount", funds.String())
	}

	txhash, err := tn.ExecTx(ctx, acc.KeyName, cmds...)
	if err != nil {
		return nil, fmt.Errorf(err.Error(), "failed to execute Contract")
	}

	var resp wasmtypes.MsgExecuteContractResponse

	txhashBytes, err := hex.DecodeString(txhash)
	if err != nil {
		return nil, err
	}
	res, err := tn.Client.Tx(ctx, txhashBytes, false)
	if err != nil {
		return nil, fmt.Errorf(err.Error(), "failed to find tx result %s", txhash)
	}
	if res.TxResult.Code != 0 {
		return nil, fmt.Errorf("tx has non-zero code (%d) with log: %s", res.TxResult.Code, res.TxResult.Log)
	}

	// Only unmarshal result if user wants to
	if &resp != nil {
		err = unmarshalTxResult(res.TxResult.Data, &resp)
		if err != nil {
			return nil, err
		}
	}

	if result != nil {
		err = json.Unmarshal(resp.Data, result)
		if err != nil {
			return nil, fmt.Errorf(err.Error(), "failed to unmarshal result")
		}
	}

	return result, nil
}

func ReadInitMessagesFile(path string) ([]*Contract, error) {
	jsonFile, err := os.Open(path)
	// if we os.Open returns an error then handle it
	if err != nil {
		return nil, err
	}
	fmt.Printf("Successfully Opened %s \n", path)

	byteValue, err := ioutil.ReadAll(jsonFile)
	if err != nil {
		return nil, err
	}

	// we initialize our Users array
	var contractDetails ContractDetails

	// we unmarshal our byteArray which contains our
	// jsonFile's content into '' which we defined above
	err = json.Unmarshal(byteValue, &contractDetails)
	if err != nil {
		return nil, err
	}

	var primitives []*Contract
	for _, im := range contractDetails.ContractDetails {
		primitives = append(primitives, NewContract(im.InitMessage, im.Label, 0))
	}

	return primitives, nil
}

func StoreContractCode(ctx context.Context, chain *cosmos.CosmosChain, filePath string, acc *ibc.Wallet, l *zap.Logger) (uint64, error) {
	// Read the Contract from os file
	contract, err := os.ReadFile(filePath)
	if err != nil {
		return 0, err
	}

	tn := GetFullNode(chain)

	logger := l.With(
		zap.String("chain_id", tn.Chain.Config().ChainID),
		zap.String("test", tn.TestName),
	)

	contractFile := "Contract.wasm"
	fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
	err = fw.WriteFile(ctx, tn.VolumeName, contractFile, contract)
	if err != nil {
		return 0, fmt.Errorf(err.Error(), "failed to write Contract file")
	}

	txhash, err := tn.ExecTx(ctx, acc.KeyName,
		"wasm", "store", filepath.Join(tn.HomeDir(), contractFile),
		"--gas", "20000000",
	)
	if err != nil {
		return 0, fmt.Errorf(err.Error(), "failed to store code")
	}

	var resp wasmtypes.MsgStoreCodeResponse
	txhashBytes, err := hex.DecodeString(txhash)
	if err != nil {
		return 0, err
	}
	res, err := tn.Client.Tx(ctx, txhashBytes, false)
	if err != nil {
		return 0, fmt.Errorf(err.Error(), "failed to find tx result %s", txhash)
	}
	if res.TxResult.Code != 0 {
		return 0, fmt.Errorf("tx has non-zero code (%d) with log: %s", res.TxResult.Code, res.TxResult.Log)
	}

	// Only unmarshal result if user wants to
	if &resp != nil {
		err = unmarshalTxResult(res.TxResult.Data, &resp)
		if err != nil {
			return 0, err
		}
	}

	return resp.CodeID, nil
}
