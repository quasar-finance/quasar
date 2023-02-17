package testutils

import (
	"encoding/json"

	"github.com/cosmos/cosmos-sdk/simapp"
	ibctesting "github.com/cosmos/ibc-go/v4/testing"
	"github.com/quasarlabs/quasarnode/app"
	"github.com/tendermint/tendermint/libs/log"
	dbm "github.com/tendermint/tm-db"
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
		simapp.EmptyAppOptions{},
		app.GetWasmEnabledProposals(),
		app.EmptyWasmOpts,
	)

	return quasarApp, app.NewDefaultGenesisState(encCdc.Marshaler)
}

// GetQuasarApp returns the current chain's app as an QuasarApp
func (chain *TestChain) GetQuasarApp() *app.App {
	v, _ := chain.App.(*app.App)
	return v
}
