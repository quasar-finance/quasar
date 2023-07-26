package suite

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/gogo/protobuf/proto"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"sync"
	"time"
)

type TestCases []*TestCase

type TestCase struct {
	Input             Input
	Output            Output
	ExpectedDeviation any
}

type Input struct {
	Account             ibc.Wallet
	Amount              sdk.Coins
	PreTxnInputCommand  []string
	TxnInput            []byte
	PostTxnInputCommand []string
	Resp                proto.Message // needs to be set by user if needed
}

type Output struct {
	RetryCount        int
	RetryInterval     time.Duration
	Result            any // needs to be set by user if needed
	PreQueryCommand   []string
	QueryCommand      []byte
	PostQueryCommand  []string
	OperationOnResult func() bool // needs to be set by user if needed
}

func NewTestCases(testCases []*TestCase) TestCases {
	return testCases
}

func (tcs *TestCases) ExecuteCases(chain *cosmos.CosmosChain, ctx context.Context) error {
	waitGroup := &sync.WaitGroup{}
	outputChannel := make(chan error, len(*tcs))

	tn := GetFullNode(chain)
	for _, t := range *tcs {
		finalTxnInput := append(t.Input.PreTxnInputCommand, string(t.Input.TxnInput))
		finalTxnInput = append(finalTxnInput, t.Input.PostTxnInputCommand...)
		if !t.Input.Amount.Empty() {
			finalTxnInput = append(finalTxnInput, "--amount", t.Input.Amount.String())
		}

		txhash, err := tn.ExecTx(ctx, t.Input.Account.KeyName, finalTxnInput...)

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
		if t.Input.Resp != nil {
			err = unmarshalTxResult(res.TxResult.Data, t.Input.Resp)
			if err != nil {
				return err
			}
		}

		waitGroup.Add(1)
		go t.GoVerify(chain, ctx, outputChannel, waitGroup)

	}

	go monitorWorker(waitGroup, outputChannel)

	done := make(chan bool, 1)
	go printWorker(outputChannel, done)
	<-done

	return nil
}

func (tc *TestCase) GoVerify(chain *cosmos.CosmosChain, ctx context.Context, oc chan error, wg *sync.WaitGroup) {
	defer wg.Done()

	// if not supplied take default as 10
	if tc.Output.RetryCount <= 0 {
		tc.Output.RetryCount = 10
	}

	// append all the whole command together
	finalQueryInput := append(tc.Output.PreQueryCommand, string(tc.Output.QueryCommand))
	finalQueryInput = append(finalQueryInput, tc.Output.PostQueryCommand...)

	// if not supplied then give default as 5
	if tc.Output.RetryInterval == time.Duration(0) {
		tc.Output.RetryInterval = time.Second * 5
	}

	for i := 1; i <= tc.Output.RetryCount; i++ {
		tn := GetFullNode(chain)

		res, _, err := tn.ExecQuery(ctx, finalQueryInput...)
		if i == tc.Output.RetryCount && err != nil {
			oc <- err
			break
		}

		if res != nil && tc.Output.Result != nil {
			err = json.Unmarshal(res, tc.Output.Result)
			if err != nil {
				oc <- err
				break
			}

			if tc.Output.OperationOnResult() {
				oc <- fmt.Errorf("test case has passed")
				break
			}
		}

		if i == tc.Output.RetryCount && err == nil {
			oc <- fmt.Errorf("could not verify the test case till end, with command `%s`, account key name `%s` and amount `%s`", string(tc.Input.TxnInput), tc.Input.Account.KeyName, tc.Input.Amount.String())
			break
		}

		time.Sleep(tc.Output.RetryInterval)
	}
}

func (tc *TestCase) ExecuteCase(chain *cosmos.CosmosChain, ctx context.Context) error {
	finalTxnInput := append(tc.Input.PreTxnInputCommand, string(tc.Input.TxnInput))
	finalTxnInput = append(finalTxnInput, tc.Input.PostTxnInputCommand...)

	if !tc.Input.Amount.Empty() {
		finalTxnInput = append(finalTxnInput, "--amount", tc.Input.Amount.String())
	}

	tn := GetFullNode(chain)
	txhash, err := tn.ExecTx(ctx, tc.Input.Account.KeyName, finalTxnInput...)

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
	if tc.Input.Resp != nil {
		err = unmarshalTxResult(res.TxResult.Data, tc.Input.Resp)
		if err != nil {
			return err
		}
	}

	err = tc.VerifyCase(chain, ctx)
	return nil
}

func (tc *TestCase) VerifyCase(chain *cosmos.CosmosChain, ctx context.Context) error {
	// if not supplied take default as 10
	if tc.Output.RetryCount <= 0 {
		tc.Output.RetryCount = 10
	}

	finalQueryInput := append(tc.Output.PreQueryCommand, string(tc.Output.QueryCommand))
	finalQueryInput = append(finalQueryInput, tc.Output.PostQueryCommand...)

	for i := 1; i <= tc.Output.RetryCount; i++ {
		tn := GetFullNode(chain)

		res, _, err := tn.ExecQuery(ctx, finalQueryInput...)
		if i == tc.Output.RetryCount && err != nil {
			return err
		}

		if res != nil && tc.Output.Result != nil {
			err = json.Unmarshal(res, tc.Output.Result)
			if err != nil {
				return err
			}

			if tc.Output.OperationOnResult() {
				return nil
			}
		}

		// if not supplied then give default as 5
		if tc.Output.RetryInterval == time.Duration(0) {
			tc.Output.RetryInterval = time.Second * 5
		}
		time.Sleep(tc.Output.RetryInterval)
	}

	return fmt.Errorf("drained all attempts at query")
}

func printWorker(cs <-chan error, done chan<- bool) {
	for i := range cs {
		fmt.Println("printing from output in all test cases:", i)
	}

	done <- true
}

func monitorWorker(wg *sync.WaitGroup, cs chan error) {
	wg.Wait()
	close(cs)
}
