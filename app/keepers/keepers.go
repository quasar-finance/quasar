package keepers

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	crisiskeeper "github.com/cosmos/cosmos-sdk/x/crisis/keeper"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	evidencekeeper "github.com/cosmos/cosmos-sdk/x/evidence/keeper"
	feegrantkeeper "github.com/cosmos/cosmos-sdk/x/feegrant/keeper"
	govkeeper "github.com/cosmos/cosmos-sdk/x/gov/keeper"
	mintkeeper "github.com/cosmos/cosmos-sdk/x/mint/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	slashingkeeper "github.com/cosmos/cosmos-sdk/x/slashing/keeper"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	upgradekeeper "github.com/cosmos/cosmos-sdk/x/upgrade/keeper"
	icacontrollerkeeper "github.com/cosmos/ibc-go/v4/modules/apps/27-interchain-accounts/controller/keeper"
	icahostkeeper "github.com/cosmos/ibc-go/v4/modules/apps/27-interchain-accounts/host/keeper"
	"github.com/cosmos/ibc-go/v4/modules/apps/transfer"
	ibctransferkeeper "github.com/cosmos/ibc-go/v4/modules/apps/transfer/keeper"
	ibckeeper "github.com/cosmos/ibc-go/v4/modules/core/keeper"
	epochsmodulekeeper "github.com/quasarlabs/quasarnode/x/epochs/keeper"
	qoraclemodulekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
	"github.com/quasarlabs/quasarnode/x/qtransfer"
	qtransferkeeper "github.com/quasarlabs/quasarnode/x/qtransfer/keeper"
	qvestingmodulekeeper "github.com/quasarlabs/quasarnode/x/qvesting/keeper"
	tfmodulekeeper "github.com/quasarlabs/quasarnode/x/tokenfactory/keeper"
)

type AppKeepers struct {
	// Special keepers
	ParamsKeeper     paramskeeper.Keeper
	CapabilityKeeper *capabilitykeeper.Keeper
	CrisisKeeper     crisiskeeper.Keeper
	UpgradeKeeper    upgradekeeper.Keeper

	// make scoped keepers public for test purposes
	ScopedIBCKeeper           capabilitykeeper.ScopedKeeper
	ScopedTransferKeeper      capabilitykeeper.ScopedKeeper
	ScopedICAControllerKeeper capabilitykeeper.ScopedKeeper
	ScopedICAHostKeeper       capabilitykeeper.ScopedKeeper
	ScopedIntergammKeeper     capabilitykeeper.ScopedKeeper
	ScopedQOracleKeeper       capabilitykeeper.ScopedKeeper
	ScopedWasmKeeper          capabilitykeeper.ScopedKeeper

	// "Normal" keepers
	AccountKeeper       authkeeper.AccountKeeper
	BankKeeper          bankkeeper.Keeper
	StakingKeeper       stakingkeeper.Keeper
	SlashingKeeper      slashingkeeper.Keeper
	MintKeeper          mintkeeper.Keeper
	DistrKeeper         distrkeeper.Keeper
	GovKeeper           govkeeper.Keeper
	IBCKeeper           *ibckeeper.Keeper // IBC Keeper must be a pointer in the app, so we can SetRouter on it correctly
	EvidenceKeeper      evidencekeeper.Keeper
	TransferKeeper      ibctransferkeeper.Keeper
	FeeGrantKeeper      feegrantkeeper.Keeper
	WasmKeeper          wasm.Keeper
	QTransferKeeper     qtransferkeeper.Keeper
	EpochsKeeper        *epochsmodulekeeper.Keeper
	QOsmosisKeeper      qosmokeeper.Keeper
	QOracleKeeper       qoraclemodulekeeper.Keeper
	QVestingKeeper      qvestingmodulekeeper.Keeper
	TfKeeper            tfmodulekeeper.Keeper
	ICAControllerKeeper icacontrollerkeeper.Keeper
	ICAHostKeeper       icahostkeeper.Keeper

	// IBC modules
	// transfer module
	RawIcs20TransferAppModule transfer.AppModule
	TransferStack             *qtransfer.IBCMiddleware
	Ics20WasmHooks            *qtransfer.WasmHooks
	HooksICS4Wrapper          qtransfer.ICS4Middleware

	// this line is used by starport scaffolding # stargate/app/keeperDeclaration
}
