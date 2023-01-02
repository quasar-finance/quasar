package testutils

import (
	"encoding/json"

	ibctesting "github.com/cosmos/ibc-go/v5/testing"
	"github.com/quasarlabs/quasarnode/app"
)

type TestChain struct {
	*ibctesting.TestChain
}

func SetupTestingApp() (ibctesting.TestingApp, map[string]json.RawMessage) {
	quasarApp := app.Setup(false)
	return quasarApp, app.NewDefaultGenesisState()
}

// GetQuasarApp returns the current chain's app as an QuasarApp
func (chain *TestChain) GetQuasarApp() *app.App {
	v, _ := chain.App.(*app.App)
	return v
}
