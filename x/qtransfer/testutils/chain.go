package testutils

import (
	"encoding/json"

	dbm "github.com/cometbft/cometbft-db"
	"github.com/cometbft/cometbft/libs/log"
	"github.com/cosmos/cosmos-sdk/testutil/sims"
	ibctesting "github.com/cosmos/ibc-go/v8/testing"

	"github.com/quasarlabs/quasarnode/app"
)

type TestChain struct {
	*ibctesting.TestChain
}

func SetupTestingApp() (ibctesting.TestingApp, map[string]json.RawMessage) {
	db := dbm.NewMemDB()
	encCdc := app.MakeEncodingConfig()
	quasarApp := app.New(
		log.NewNopLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		app.DefaultNodeHome,
		5,
		encCdc,
		sims.EmptyAppOptions{},
		app.EmptyWasmOpts,
	)

	return quasarApp, app.NewDefaultGenesisState(encCdc.Marshaler)
}

// GetQuasarApp returns the current chain's app as an QuasarApp
func (chain *TestChain) GetQuasarApp() *app.QuasarApp {
	v, _ := chain.App.(*app.QuasarApp)
	return v
}
