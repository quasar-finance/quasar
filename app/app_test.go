package app_test

import (
	"os"
	"testing"

	"cosmossdk.io/log"
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	db "github.com/cosmos/cosmos-db"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	"github.com/quasar-finance/quasar/app"
	"github.com/quasar-finance/quasar/testutil"
	"github.com/stretchr/testify/require"
)

type EmptyAppOptions struct{}

var emptyWasmOption []wasmkeeper.Option

var tempDir = func() string {
	dir, err := os.MkdirTemp("", ".quasarnode")
	if err != nil {
		dir = app.DefaultNodeHome
	}
	defer os.RemoveAll(dir)

	return dir
}

func (ao EmptyAppOptions) Get(_ string) interface{} {
	return nil
}

func TestQuasarApp_BlockedModuleAccountAddrs(t *testing.T) {
	app := app.New(
		log.NewNopLogger(),
		db.NewMemDB(),
		nil,
		true,
		map[int64]bool{},
		tempDir(),
		0,
		EmptyAppOptions{},
		emptyWasmOption,
	)

	blockedAddrs := app.BlockedModuleAccountAddrs()

	require.NotContains(t, blockedAddrs, authtypes.NewModuleAddress(govtypes.ModuleName).String())
}

func TestQuasarApp_Export(t *testing.T) {
	app := testutil.Setup(t)
	_, err := app.ExportAppStateAndValidators(true, []string{}, []string{})
	require.NoError(t, err, "ExportAppStateAndValidators should not have an error")
}
