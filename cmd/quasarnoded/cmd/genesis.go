package cmd

import (
	"encoding/json"
	"fmt"
	"time"

	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmtypes "github.com/tendermint/tendermint/types"

	"github.com/cosmos/cosmos-sdk/client"
	sdk "github.com/cosmos/cosmos-sdk/types"

	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	crisistypes "github.com/cosmos/cosmos-sdk/x/crisis/types"
	distributiontypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	appParams "github.com/abag/quasarnode/app/params"
	epochstypes "github.com/abag/quasarnode/x/epochs/types"
	oriontypes "github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	qoracletypes "github.com/abag/quasarnode/x/qoracle/types"
)

func PrepareGenesis(clientCtx client.Context, appState map[string]json.RawMessage, genDoc *tmtypes.GenesisDoc, genesisParams GenesisParams, chainID string) (map[string]json.RawMessage, *tmtypes.GenesisDoc, error) {
	depCdc := clientCtx.Codec
	cdc := depCdc

	// chain params genesis
	genDoc.ChainID = chainID
	genDoc.GenesisTime = genesisParams.GenesisTime

	genDoc.ConsensusParams = genesisParams.ConsensusParams

	// ---
	// staking module genesis
	stakingGenState := stakingtypes.GetGenesisStateFromAppState(depCdc, appState)
	stakingGenState.Params = genesisParams.StakingParams
	stakingGenStateBz, err := cdc.MarshalJSON(stakingGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal staking genesis state: %w", err)
	}
	appState[stakingtypes.ModuleName] = stakingGenStateBz

	// mint module genesis
	mintGenState := minttypes.DefaultGenesisState()
	mintGenState.Params = genesisParams.MintParams
	mintGenStateBz, err := cdc.MarshalJSON(mintGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal mint genesis state: %w", err)
	}
	appState[minttypes.ModuleName] = mintGenStateBz

	// distribution module genesis
	distributionGenState := distributiontypes.DefaultGenesisState()
	distributionGenState.Params = genesisParams.DistributionParams
	distributionGenStateBz, err := cdc.MarshalJSON(distributionGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal distribution genesis state: %w", err)
	}
	appState[distributiontypes.ModuleName] = distributionGenStateBz

	// gov module genesis
	govGenState := govtypes.DefaultGenesisState()
	govGenState.DepositParams = genesisParams.GovParams.DepositParams
	govGenState.TallyParams = genesisParams.GovParams.TallyParams
	govGenState.VotingParams = genesisParams.GovParams.VotingParams
	govGenStateBz, err := cdc.MarshalJSON(govGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal gov genesis state: %w", err)
	}
	appState[govtypes.ModuleName] = govGenStateBz

	// crisis module genesis
	crisisGenState := crisistypes.DefaultGenesisState()
	crisisGenState.ConstantFee = genesisParams.CrisisConstantFee
	crisisGenStateBz, err := cdc.MarshalJSON(crisisGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal crisis genesis state: %w", err)
	}
	appState[crisistypes.ModuleName] = crisisGenStateBz

	// slashing module genesis
	slashingGenState := slashingtypes.DefaultGenesisState()
	slashingGenState.Params = genesisParams.SlashingParams
	slashingGenStateBz, err := cdc.MarshalJSON(slashingGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal slashing genesis state: %w", err)
	}
	appState[slashingtypes.ModuleName] = slashingGenStateBz

	// epochs module genesis
	epochsGenState := epochstypes.DefaultGenesis()
	epochsGenState.Epochs = genesisParams.Epochs
	epochsGenStateBz, err := cdc.MarshalJSON(epochsGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal epochs genesis state: %w", err)
	}
	appState[epochstypes.ModuleName] = epochsGenStateBz

	// orion module genesis
	orionGenState := oriontypes.DefaultGenesis()
	orionGenState.Params = genesisParams.OrionParams
	orionGenStateBz, err := cdc.MarshalJSON(orionGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal orion genesis state: %w", err)
	}
	appState[oriontypes.ModuleName] = orionGenStateBz

	// orion module genesis
	qBankGenState := qbanktypes.DefaultGenesis()
	qBankGenState.Params = genesisParams.QBankParams
	qBankGenStateBz, err := cdc.MarshalJSON(qBankGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal qbank genesis state: %w", err)
	}
	appState[qbanktypes.ModuleName] = qBankGenStateBz

	// orion module genesis
	qOracleGenState := qoracletypes.DefaultGenesis()
	qOracleGenState.Params = genesisParams.QOracleParams
	qOracleGenStateBz, err := cdc.MarshalJSON(qOracleGenState)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal qoracle genesis state: %w", err)
	}
	appState[qoracletypes.ModuleName] = qOracleGenStateBz

	// return appState and genDoc
	return appState, genDoc, nil
}

type GenesisParams struct {
	AirdropSupply sdk.Int

	StrategicReserveAccounts []banktypes.Balance

	ConsensusParams *tmproto.ConsensusParams

	GenesisTime         time.Time
	NativeCoinMetadatas []banktypes.Metadata

	StakingParams      stakingtypes.Params
	MintParams         minttypes.Params
	DistributionParams distributiontypes.Params
	GovParams          govtypes.Params

	CrisisConstantFee sdk.Coin

	SlashingParams slashingtypes.Params

	Epochs []epochstypes.EpochInfo

	OrionParams   oriontypes.Params
	QBankParams   qbanktypes.Params
	QOracleParams qoracletypes.Params
}

func MainnetGenesisParams() GenesisParams {
	genParams := GenesisParams{}

	genParams.AirdropSupply = sdk.NewIntWithDecimal(5, 13)                // 5*10^13 uqsar, 5*10^7 (50 million) qsar
	genParams.GenesisTime = time.Date(2021, 6, 18, 17, 0, 0, 0, time.UTC) // Jun 18, 2021 - 17:00 UTC

	genParams.NativeCoinMetadatas = []banktypes.Metadata{
		{
			Description: "The native token of Quasar",
			DenomUnits: []*banktypes.DenomUnit{
				{
					Denom:    appParams.BaseCoinUnit,
					Exponent: 0,
					Aliases:  nil,
				},
				{
					Denom:    appParams.HumanCoinUnit,
					Exponent: appParams.QsrExponent,
					Aliases:  nil,
				},
			},
			Base:    appParams.BaseCoinUnit,
			Display: appParams.HumanCoinUnit,
		},
		{
			DenomUnits: []*banktypes.DenomUnit{
				{
					Denom:    "uorion",
					Exponent: 0,
					Aliases:  nil,
				},
				{
					Denom:    "orion",
					Exponent: 6,
					Aliases:  nil,
				},
			},
			Base:    "uorion",
			Display: "orion",
		},
	}

	genParams.StrategicReserveAccounts = []banktypes.Balance{
		{
			Address: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
			Coins:   sdk.NewCoins(sdk.NewCoin(genParams.NativeCoinMetadatas[0].Base, sdk.NewInt(500_000_000))), // oracle account 500 QSR for fees
		},
	}

	genParams.StakingParams = stakingtypes.DefaultParams()
	genParams.StakingParams.UnbondingTime = time.Hour * 24 * 7 * 2 // 2 weeks
	genParams.StakingParams.MaxValidators = 100
	genParams.StakingParams.BondDenom = genParams.NativeCoinMetadatas[0].Base

	genParams.MintParams = minttypes.DefaultParams()
	genParams.MintParams.MintDenom = genParams.NativeCoinMetadatas[0].Base

	genParams.DistributionParams = distributiontypes.DefaultParams()
	genParams.DistributionParams.BaseProposerReward = sdk.MustNewDecFromStr("0.01")
	genParams.DistributionParams.BonusProposerReward = sdk.MustNewDecFromStr("0.04")
	genParams.DistributionParams.CommunityTax = sdk.MustNewDecFromStr("0")
	genParams.DistributionParams.WithdrawAddrEnabled = true

	genParams.GovParams = govtypes.DefaultParams()
	genParams.GovParams.DepositParams.MaxDepositPeriod = time.Hour * 24 * 14 // 2 weeks
	genParams.GovParams.DepositParams.MinDeposit = sdk.NewCoins(sdk.NewCoin(
		genParams.NativeCoinMetadatas[0].Base,
		sdk.NewInt(2_500_000_000),
	))
	genParams.GovParams.TallyParams.Quorum = sdk.MustNewDecFromStr("0.2") // 20%
	genParams.GovParams.VotingParams.VotingPeriod = time.Hour * 24 * 3    // 3 days

	genParams.CrisisConstantFee = sdk.NewCoin(
		genParams.NativeCoinMetadatas[0].Base,
		sdk.NewInt(500_000_000_000),
	)

	genParams.SlashingParams = slashingtypes.DefaultParams()
	genParams.SlashingParams.SignedBlocksWindow = int64(30000)                       // 30000 blocks (~41 hr at 5 second blocks)
	genParams.SlashingParams.MinSignedPerWindow = sdk.MustNewDecFromStr("0.05")      // 5% minimum liveness
	genParams.SlashingParams.DowntimeJailDuration = time.Minute                      // 1 minute jail period
	genParams.SlashingParams.SlashFractionDoubleSign = sdk.MustNewDecFromStr("0.05") // 5% double sign slashing
	genParams.SlashingParams.SlashFractionDowntime = sdk.ZeroDec()                   // 0% liveness slashing

	genParams.Epochs = epochstypes.DefaultGenesis().Epochs
	for _, epoch := range genParams.Epochs {
		epoch.StartTime = genParams.GenesisTime
	}

	genParams.ConsensusParams = tmtypes.DefaultConsensusParams()
	genParams.ConsensusParams.Block.MaxBytes = 5 * 1024 * 1024
	genParams.ConsensusParams.Block.MaxGas = 6_000_000
	genParams.ConsensusParams.Evidence.MaxAgeDuration = genParams.StakingParams.UnbondingTime
	genParams.ConsensusParams.Evidence.MaxAgeNumBlocks = int64(genParams.StakingParams.UnbondingTime.Seconds()) / 3
	genParams.ConsensusParams.Version.AppVersion = 1

	genParams.OrionParams = oriontypes.DefaultParams()
	genParams.OrionParams.Enabled = true
	genParams.OrionParams.LpEpochId = "day"
	genParams.OrionParams.MgmtFeePer = sdk.MustNewDecFromStr("0.003")
	genParams.OrionParams.PerfFeePer = sdk.MustNewDecFromStr("0.02")

	genParams.QBankParams = qbanktypes.DefaultParams()
	genParams.QBankParams.Enabled = true
	genParams.QBankParams.MinOrionEpochDenomDollarDeposit = sdk.NewDec(100)
	genParams.QBankParams.OrionEpochIdentifier = "day"
	genParams.QBankParams.WhiteListedDenomsInOrion = []qbanktypes.WhiteListedDenomInOrion{
		// TODO recheck
		{
			OriginName:   "uatom",
			OnehopQuasar: "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
			OnehopOsmo:   "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
		},
		{
			OriginName:   "uqsar",
			OnehopQuasar: "uqsar",
			OnehopOsmo:   "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
		},
	}

	return genParams
}

func TestnetGenesisParams() GenesisParams {
	genParams := MainnetGenesisParams()

	genParams.GenesisTime = time.Now().UTC()

	genParams.Epochs = append(genParams.Epochs, epochstypes.EpochInfo{
		Identifier:            "15min",
		StartTime:             time.Time{},
		Duration:              15 * time.Minute,
		CurrentEpoch:          0,
		CurrentEpochStartTime: time.Time{},
		EpochCountingStarted:  false,
	})

	for _, epoch := range genParams.Epochs {
		epoch.StartTime = genParams.GenesisTime
	}

	genParams.StakingParams.UnbondingTime = time.Hour * 24 * 7 * 2 // 2 weeks

	genParams.GovParams.DepositParams.MinDeposit = sdk.NewCoins(sdk.NewCoin(
		genParams.NativeCoinMetadatas[0].Base,
		sdk.NewInt(1_000_000), // 1 QSR
	))
	genParams.GovParams.TallyParams.Quorum = sdk.MustNewDecFromStr("0.0000001") // 0.00001%
	genParams.GovParams.VotingParams.VotingPeriod = time.Second * 20            // 20 seconds

	genParams.OrionParams.Enabled = false
	genParams.OrionParams.LpEpochId = "minute"

	genParams.QBankParams.OrionEpochIdentifier = "minute"

	return genParams
}
