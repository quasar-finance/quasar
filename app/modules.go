package app

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	"github.com/cosmos/cosmos-sdk/x/auth"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	"github.com/cosmos/cosmos-sdk/x/auth/vesting"
	vestingtypes "github.com/cosmos/cosmos-sdk/x/auth/vesting/types"
	authztypes "github.com/cosmos/cosmos-sdk/x/authz"
	authzmodule "github.com/cosmos/cosmos-sdk/x/authz/module"
	"github.com/cosmos/cosmos-sdk/x/bank"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	"github.com/cosmos/cosmos-sdk/x/capability"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	"github.com/cosmos/cosmos-sdk/x/consensus"
	consensusparamtypes "github.com/cosmos/cosmos-sdk/x/consensus/types"
	"github.com/cosmos/cosmos-sdk/x/crisis"
	crisistypes "github.com/cosmos/cosmos-sdk/x/crisis/types"
	distr "github.com/cosmos/cosmos-sdk/x/distribution"
	distrtypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	"github.com/cosmos/cosmos-sdk/x/evidence"
	evidencetypes "github.com/cosmos/cosmos-sdk/x/evidence/types"
	"github.com/cosmos/cosmos-sdk/x/feegrant"
	feegrantmodule "github.com/cosmos/cosmos-sdk/x/feegrant/module"
	"github.com/cosmos/cosmos-sdk/x/genutil"
	genutiltypes "github.com/cosmos/cosmos-sdk/x/genutil/types"
	"github.com/cosmos/cosmos-sdk/x/gov"
	govclient "github.com/cosmos/cosmos-sdk/x/gov/client"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	"github.com/cosmos/cosmos-sdk/x/mint"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	"github.com/cosmos/cosmos-sdk/x/params"
	paramsclient "github.com/cosmos/cosmos-sdk/x/params/client"
	paramstypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"github.com/cosmos/cosmos-sdk/x/slashing"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	"github.com/cosmos/cosmos-sdk/x/staking"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	"github.com/cosmos/cosmos-sdk/x/upgrade"
	upgradeclient "github.com/cosmos/cosmos-sdk/x/upgrade/client"
	upgradetypes "github.com/cosmos/cosmos-sdk/x/upgrade/types"
	ibcwasm "github.com/cosmos/ibc-go/modules/light-clients/08-wasm"
	ibcwasmtypes "github.com/cosmos/ibc-go/modules/light-clients/08-wasm/types"
	ica "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts"
	icatypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/types"
	"github.com/cosmos/ibc-go/v7/modules/apps/transfer"
	ibctransfertypes "github.com/cosmos/ibc-go/v7/modules/apps/transfer/types"
	ibc "github.com/cosmos/ibc-go/v7/modules/core"
	ibcclientclient "github.com/cosmos/ibc-go/v7/modules/core/02-client/client"
	ibcexported "github.com/cosmos/ibc-go/v7/modules/core/exported"
	ibchost "github.com/cosmos/ibc-go/v7/modules/core/exported"
	ibctm "github.com/cosmos/ibc-go/v7/modules/light-clients/07-tendermint"

	appparams "github.com/quasarlabs/quasarnode/app/params"
	epochsmodule "github.com/quasarlabs/quasarnode/x/epochs"
	epochsmoduletypes "github.com/quasarlabs/quasarnode/x/epochs/types"
	qoraclemodule "github.com/quasarlabs/quasarnode/x/qoracle"
	qoraclemoduletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/quasarlabs/quasarnode/x/qtransfer"
	qtransfertypes "github.com/quasarlabs/quasarnode/x/qtransfer/types"
	qvestingmodule "github.com/quasarlabs/quasarnode/x/qvesting"
	qvestingmoduletypes "github.com/quasarlabs/quasarnode/x/qvesting/types"
	tfmodule "github.com/quasarlabs/quasarnode/x/tokenfactory"
	tftypes "github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

// moduleAccountPermissions defines module account permissions
var ModuleAccountPermissions = map[string][]string{
	authtypes.FeeCollectorName:     nil,
	distrtypes.ModuleName:          nil,
	icatypes.ModuleName:            nil,
	minttypes.ModuleName:           {authtypes.Minter, authtypes.Burner},
	stakingtypes.BondedPoolName:    {authtypes.Burner, authtypes.Staking},
	stakingtypes.NotBondedPoolName: {authtypes.Burner, authtypes.Staking},
	govtypes.ModuleName:            {authtypes.Burner},
	ibctransfertypes.ModuleName:    {authtypes.Minter, authtypes.Burner},
	wasmtypes.ModuleName:           {authtypes.Burner},
	tftypes.ModuleName:             {authtypes.Minter, authtypes.Burner},
}

// AppModuleBasics returns ModuleBasics for the module BasicManager.
var AppModuleBasics = []module.AppModuleBasic{
	auth.AppModuleBasic{},
	genutil.AppModuleBasic{GenTxValidator: genutiltypes.DefaultMessageValidator},
	bank.AppModuleBasic{},
	capability.AppModuleBasic{},
	staking.AppModuleBasic{},
	mint.AppModuleBasic{},
	distr.AppModuleBasic{},
	gov.NewAppModuleBasic(
		[]govclient.ProposalHandler{
			paramsclient.ProposalHandler,
			upgradeclient.LegacyProposalHandler,
			upgradeclient.LegacyProposalHandler,
			ibcclientclient.UpdateClientProposalHandler,
			ibcclientclient.UpgradeProposalHandler,
		},
	),
	params.AppModuleBasic{},
	crisis.AppModuleBasic{},
	slashing.AppModuleBasic{},
	feegrantmodule.AppModuleBasic{},
	ibc.AppModuleBasic{},
	upgrade.AppModuleBasic{},
	evidence.AppModuleBasic{},
	transfer.AppModuleBasic{},
	vesting.AppModuleBasic{},
	epochsmodule.AppModuleBasic{},
	qoraclemodule.AppModuleBasic{},
	ica.AppModuleBasic{},
	ibcwasm.AppModuleBasic{},
	wasm.AppModuleBasic{},
	qtransfer.AppModuleBasic{},
	tfmodule.AppModuleBasic{},
	qvestingmodule.AppModuleBasic{},
	authzmodule.AppModuleBasic{},
	consensus.AppModuleBasic{},
	ibctm.AppModuleBasic{},
}

func appModules(
	app *QuasarApp,
	encodingConfig appparams.EncodingConfig,
	skipGenesisInvariants bool,
) []module.AppModule {
	appCodec := encodingConfig.Marshaler

	return []module.AppModule{
		genutil.NewAppModule(
			app.AccountKeeper, app.StakingKeeper, app.BaseApp.DeliverTx,
			encodingConfig.TxConfig,
		),
		auth.NewAppModule(appCodec, app.AccountKeeper, nil, app.GetSubspace(authtypes.ModuleName)),
		vesting.NewAppModule(app.AccountKeeper, app.BankKeeper),
		bank.NewAppModule(appCodec, app.BankKeeper, app.AccountKeeper, app.GetSubspace(banktypes.ModuleName)),
		capability.NewAppModule(appCodec, *app.CapabilityKeeper, false),
		crisis.NewAppModule(app.CrisisKeeper, skipGenesisInvariants, app.GetSubspace(crisistypes.ModuleName)),
		gov.NewAppModule(appCodec, &app.GovKeeper, app.AccountKeeper, app.BankKeeper, app.GetSubspace(govtypes.ModuleName)),
		mint.NewAppModule(appCodec, app.MintKeeper, app.AccountKeeper, nil, app.GetSubspace(minttypes.ModuleName)),
		slashing.NewAppModule(appCodec, app.SlashingKeeper, app.AccountKeeper, app.BankKeeper, app.StakingKeeper, app.GetSubspace(slashingtypes.ModuleName)),
		distr.NewAppModule(appCodec, app.DistrKeeper, app.AccountKeeper, app.BankKeeper, app.StakingKeeper, app.GetSubspace(distrtypes.ModuleName)),
		staking.NewAppModule(appCodec, app.StakingKeeper, app.AccountKeeper, app.BankKeeper, app.GetSubspace(stakingtypes.ModuleName)),
		upgrade.NewAppModule(app.UpgradeKeeper),
		ibcwasm.NewAppModule(*app.IBCWasmClientKeeper),
		evidence.NewAppModule(app.EvidenceKeeper),
		feegrantmodule.NewAppModule(appCodec, app.AccountKeeper, app.BankKeeper, app.FeeGrantKeeper, app.interfaceRegistry),
		ibc.NewAppModule(app.IBCKeeper),
		params.NewAppModule(app.ParamsKeeper),
		wasm.NewAppModule(appCodec, app.WasmKeeper, app.StakingKeeper, app.AccountKeeper, app.BankKeeper, app.MsgServiceRouter(), app.GetSubspace(wasmtypes.ModuleName)),
		app.RawIcs20TransferAppModule,
		ica.NewAppModule(&app.ICAControllerKeeper, app.ICAHostKeeper),
		authzmodule.NewAppModule(appCodec, app.AuthzKeeper, app.AccountKeeper, app.BankKeeper, app.interfaceRegistry),
		consensus.NewAppModule(appCodec, *app.ConsensusParamsKeeper),

		// quasar modules
		epochsmodule.NewAppModule(appCodec, app.EpochsKeeper),
		qoraclemodule.NewAppModule(appCodec, app.QOracleKeeper, app.QOsmosisKeeper),
		qtransfer.NewAppModule(app.QTransferKeeper),
		tfmodule.NewAppModule(app.TfKeeper, app.AccountKeeper, app.BankKeeper),
		qvestingmodule.NewAppModule(appCodec, app.QVestingKeeper, app.AccountKeeper, app.BankKeeper),
		// this line is used by starport scaffolding # stargate/app/appModule
	}
}

func orderBeginBlockers() []string {
	return []string{
		upgradetypes.ModuleName,
		capabilitytypes.ModuleName,
		epochsmoduletypes.ModuleName,
		minttypes.ModuleName,
		distrtypes.ModuleName,
		slashingtypes.ModuleName,
		evidencetypes.ModuleName,
		stakingtypes.ModuleName,
		authtypes.ModuleName,
		banktypes.ModuleName,
		govtypes.ModuleName,
		crisistypes.ModuleName,
		ibcexported.ModuleName,
		ibctransfertypes.ModuleName,
		icatypes.ModuleName,
		genutiltypes.ModuleName,
		authztypes.ModuleName,
		feegrant.ModuleName,
		paramstypes.ModuleName,
		ibchost.ModuleName,
		vestingtypes.ModuleName,
		consensusparamtypes.ModuleName,
		qoraclemoduletypes.ModuleName,
		qtransfertypes.ModuleName,
		wasmtypes.ModuleName,
		ibcwasmtypes.ModuleName,
		tftypes.ModuleName,
		qvestingmoduletypes.ModuleName,
	}
}

func orderEndBlockers() []string {
	return []string{
		crisistypes.ModuleName,
		govtypes.ModuleName,
		stakingtypes.ModuleName,
		ibcexported.ModuleName,
		qoraclemoduletypes.ModuleName,
		qtransfertypes.ModuleName,
		evidencetypes.ModuleName,
		ibchost.ModuleName,
		feegrant.ModuleName,
		minttypes.ModuleName,
		slashingtypes.ModuleName,
		ibctransfertypes.ModuleName,
		vestingtypes.ModuleName,
		capabilitytypes.ModuleName,
		upgradetypes.ModuleName,
		paramstypes.ModuleName,
		authtypes.ModuleName,
		banktypes.ModuleName,
		distrtypes.ModuleName,
		icatypes.ModuleName,
		genutiltypes.ModuleName,
		epochsmoduletypes.ModuleName,
		wasmtypes.ModuleName,
		ibcwasmtypes.ModuleName,
		tftypes.ModuleName,
		qvestingmoduletypes.ModuleName,
		authztypes.ModuleName,
		consensusparamtypes.ModuleName,
	}
}

/*
NOTE: The genutils module must occur after staking so that pools are
properly initialized with tokens from genesis accounts.
NOTE: The genutils module must also occur after auth so that it can access the params from auth.
NOTE: Capability module must occur first so that it can initialize any capabilities
so that other modules that want to create or claim capabilities afterwards in InitChain
can do so safely.
*/
func orderInitBlockers() []string {
	return []string{
		capabilitytypes.ModuleName,
		authtypes.ModuleName,
		banktypes.ModuleName,
		distrtypes.ModuleName,
		stakingtypes.ModuleName,
		slashingtypes.ModuleName,
		govtypes.ModuleName,
		minttypes.ModuleName,
		crisistypes.ModuleName,
		ibchost.ModuleName,
		genutiltypes.ModuleName,
		evidencetypes.ModuleName,
		ibctransfertypes.ModuleName,
		ibcexported.ModuleName,
		qoraclemoduletypes.ModuleName,
		icatypes.ModuleName,
		vestingtypes.ModuleName,
		feegrant.ModuleName,
		upgradetypes.ModuleName,
		paramstypes.ModuleName,
		epochsmoduletypes.ModuleName,
		// wasm after ibc transfer
		wasmtypes.ModuleName,
		ibcwasmtypes.ModuleName,
		qtransfertypes.ModuleName,
		tftypes.ModuleName,
		qvestingmoduletypes.ModuleName,
		authztypes.ModuleName,
		consensusparamtypes.ModuleName,
	}
}
