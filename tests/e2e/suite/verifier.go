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
	Account ibc.Wallet
	Amount  sdk.Coins
	Command []string
	Resp    proto.Message
}

type Output struct {
	RetryCount        int
	RetryInterval     time.Duration
	Result            any
	QueryCommand      []string
	OperationOnResult func() bool
}

func NewTestCases(testCases []*TestCase) TestCases {
	return testCases
}

func (tcs *TestCases) ExecuteCases(chain *cosmos.CosmosChain, ctx context.Context) error {
	waitGroup := &sync.WaitGroup{}
	outputChannel := make(chan error, len(*tcs))

	tn := GetFullNode(chain)
	for _, t := range *tcs {
		if !t.Input.Amount.Empty() {
			t.Input.Command = append(t.Input.Command, "--amount", t.Input.Amount.String())
		}

		txhash, err := tn.ExecTx(ctx, t.Input.Account.KeyName, t.Input.Command...)

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

	for i := 1; i <= tc.Output.RetryCount; i++ {
		tn := GetFullNode(chain)

		res, _, err := tn.ExecQuery(ctx, tc.Output.QueryCommand...)
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

		// if not supplied then give default as 5
		if tc.Output.RetryInterval == time.Duration(0) {
			tc.Output.RetryInterval = time.Second * 5
		}
		time.Sleep(tc.Output.RetryInterval)
	}
}

func (tc *TestCase) ExecuteCase(chain *cosmos.CosmosChain, ctx context.Context) error {
	if !tc.Input.Amount.Empty() {
		tc.Input.Command = append(tc.Input.Command, "--amount", tc.Input.Amount.String())
	}

	tn := GetFullNode(chain)
	txhash, err := tn.ExecTx(ctx, tc.Input.Account.KeyName, tc.Input.Command...)

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

	for i := 1; i <= tc.Output.RetryCount; i++ {
		tn := GetFullNode(chain)

		res, _, err := tn.ExecQuery(ctx, tc.Output.QueryCommand...)
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
