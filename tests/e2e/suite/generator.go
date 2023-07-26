package suite

import (
	"encoding/json"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"time"
)

type WasmTestCasesForGenerator []WasmTestCaseForGenerator

type WasmTestCaseForGenerator struct {
	Amount          int64  `json:"amount"`
	Denom           string `json:"denom"`
	TxnInput        any    `json:"txn_input"`
	QueryInput      any    `json:"query_input"`
	ContractAddress string `json:"contract_address"`
	RetryCount      int    `json:"retry_count"`
	RetryInterval   int    `json:"retry_interval"`
}

func (t WasmTestCasesForGenerator) ConvertToTestCases() (TestCases, error) {
	var testCases TestCases
	for _, tc := range t {
		txnInputBz, err := json.Marshal(tc.TxnInput)
		if err != nil {
			return nil, err
		}
		queryCommandBz, err := json.Marshal(tc.QueryInput)
		if err != nil {
			return nil, err
		}
		testCases = append(testCases, &TestCase{
			Input: Input{
				Account:  ibc.Wallet{}, // TODO create an automatic assigning function here
				Amount:   sdk.NewCoins(sdk.NewInt64Coin(tc.Denom, tc.Amount)),
				TxnInput: txnInputBz,
			},
			Output: Output{
				RetryCount:    tc.RetryCount,
				RetryInterval: time.Duration(tc.RetryInterval),
				QueryCommand:  queryCommandBz,
			},
		})
	}

	return testCases, nil
}

func (t *WasmTestCaseForGenerator) ConvertToTestCase() {

}
