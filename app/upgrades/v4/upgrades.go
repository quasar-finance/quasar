package v4

import (
	"context"
	"fmt"
	"sort"

	upgradetypes "cosmossdk.io/x/upgrade/types"
	comettypes "github.com/cometbft/cometbft/proto/tendermint/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	consensuskeeper "github.com/cosmos/cosmos-sdk/x/consensus/keeper"
	"github.com/cosmos/cosmos-sdk/x/consensus/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	"github.com/quasar-finance/quasar/app/keepers"
	slinkyconstants "github.com/skip-mev/slinky/cmd/constants/marketmaps"
	marketmapkeeper "github.com/skip-mev/slinky/x/marketmap/keeper"
	marketmaptypes "github.com/skip-mev/slinky/x/marketmap/types"
)

func CreateUpgradeHandler(
	mm *module.Manager,
	configurator module.Configurator,
	keepers *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(c context.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		ctx := sdk.UnwrapSDKContext(c)

		ctx.Logger().Info("Starting module migrations...")
		vm, err := mm.RunMigrations(ctx, configurator, fromVM)
		if err != nil {
			return vm, err
		}

		ctx.Logger().Info("Setting consensus params...")
		err = enableVoteExtensions(ctx, keepers.ConsensusParamsKeeper)
		if err != nil {
			return nil, err
		}

		ctx.Logger().Info("Setting marketmap params...")
		err = setMarketMapParams(ctx, keepers.MarketmapKeeper)
		if err != nil {
			return nil, err
		}

		ctx.Logger().Info("Setting marketmap and oracle state...")
		err = setMarketState(ctx, keepers.MarketmapKeeper)
		if err != nil {
			return nil, err
		}

		ctx.Logger().Info(fmt.Sprintf("Migration {%s} applied", UpgradeName))
		return vm, nil
	}
}

func setMarketMapParams(ctx sdk.Context, marketmapKeeper *marketmapkeeper.Keeper) error {
	marketmapParams := marketmaptypes.Params{
		MarketAuthorities: []string{authtypes.NewModuleAddress(govtypes.ModuleName).String()}, //MarketMapAuthorityMultisig},
		Admin:             authtypes.NewModuleAddress(govtypes.ModuleName).String(),           // TODO: decide admin
	}
	return marketmapKeeper.SetParams(ctx, marketmapParams)
}
func setMarketState(ctx sdk.Context, mmKeeper *marketmapkeeper.Keeper) error {
	markets := marketMapToDeterministicallyOrderedMarkets(slinkyconstants.CoreMarketMap)
	for _, market := range markets {
		if err := mmKeeper.CreateMarket(ctx, market); err != nil {
			return err
		}

		if err := mmKeeper.Hooks().AfterMarketCreated(ctx, market); err != nil {
			return err
		}

	}
	return nil
}

func marketMapToDeterministicallyOrderedMarkets(mm marketmaptypes.MarketMap) []marketmaptypes.Market {
	markets := make([]marketmaptypes.Market, 0, len(mm.Markets))
	for _, market := range mm.Markets {
		markets = append(markets, market)
	}

	// order the markets alphabetically by their ticker.String()
	sort.Slice(markets, func(i, j int) bool {
		return markets[i].Ticker.String() < markets[j].Ticker.String()
	})

	return markets
}

func enableVoteExtensions(ctx sdk.Context, consensusKeeper *consensuskeeper.Keeper) error {
	oldParams, err := consensusKeeper.Params(ctx, &types.QueryParamsRequest{})
	if err != nil {
		return err
	}

	oldParams.Params.Version = &comettypes.VersionParams{App: 0}
	if err := consensusKeeper.ParamsStore.Set(ctx, *oldParams.Params); err != nil {
		return err
	}

	// we need to enable VoteExtensions for Slinky
	oldParams.Params.Abci = &comettypes.ABCIParams{VoteExtensionsEnableHeight: ctx.BlockHeight() + 4}

	updateParamsMsg := types.MsgUpdateParams{
		Authority: authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		Block:     oldParams.Params.Block,
		Evidence:  oldParams.Params.Evidence,
		Validator: oldParams.Params.Validator,
		Abci:      oldParams.Params.Abci,
	}

	_, err = consensusKeeper.UpdateParams(ctx, &updateParamsMsg)
	return err
}
