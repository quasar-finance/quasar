package app

import (
	"testing"

	"github.com/abag/quasarnode/app"
	"github.com/cosmos/cosmos-sdk/simapp"
	"github.com/tendermint/starport/starport/pkg/cosmoscmd"
	"github.com/tendermint/tendermint/libs/log"
	tmdb "github.com/tendermint/tm-db"
)

func CreateTestApp(t testing.TB) *app.App {
	db := tmdb.NewMemDB()
	return app.NewQuasarApp(
		log.TestingLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		t.TempDir(),
		5,
		cosmoscmd.MakeEncodingConfig(app.ModuleBasics),
		simapp.EmptyAppOptions{},
	)
}
