package testutils

import (
	"fmt"
	"os"

	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	govtypesv1 "github.com/cosmos/cosmos-sdk/x/gov/types/v1"
	transfertypes "github.com/cosmos/ibc-go/v5/modules/apps/transfer/types"
	"github.com/stretchr/testify/suite"
)

func (chain *TestChain) StoreContractCode(suite *suite.Suite, path string) {
	quasarApp := chain.GetQuasarApp()

	govKeeper := quasarApp.GovKeeper
	wasmCode, err := os.ReadFile(path)
	suite.Require().NoError(err)

	addr := quasarApp.AccountKeeper.GetModuleAddress(govtypes.ModuleName)
	src := wasmtypes.StoreCodeProposalFixture(func(p *wasmtypes.StoreCodeProposal) {
		p.RunAs = addr.String()
		p.WASMByteCode = wasmCode
	})

	// when stored
	contentMsg, err := govtypesv1.NewLegacyContent(src, addr.String())
	suite.Require().NoError(err)
	_, err = govKeeper.SubmitProposal(chain.GetContext(), []sdk.Msg{contentMsg}, "")
	suite.Require().NoError(err)

	// and proposal execute
	handler := govKeeper.LegacyRouter().GetRoute(src.ProposalRoute())
	err = handler(chain.GetContext(), src)
	suite.Require().NoError(err)
}

func (chain *TestChain) InstantiateRLContract(suite *suite.Suite, quotas string) sdk.AccAddress {
	quasarApp := chain.GetQuasarApp()
	transferModule := quasarApp.AccountKeeper.GetModuleAddress(transfertypes.ModuleName)
	govModule := quasarApp.AccountKeeper.GetModuleAddress(govtypes.ModuleName)

	initMsgBz := []byte(fmt.Sprintf(`{
           "gov_module":  "%s",
           "ibc_module":"%s",
           "paths": [%s]
        }`,
		govModule, transferModule, quotas))

	contractKeeper := wasmkeeper.NewDefaultPermissionKeeper(quasarApp.WasmKeeper)
	codeID := uint64(1)
	creator := quasarApp.AccountKeeper.GetModuleAddress(govtypes.ModuleName)
	addr, _, err := contractKeeper.Instantiate(chain.GetContext(), codeID, creator, creator, initMsgBz, "rate limiting contract", nil)
	suite.Require().NoError(err)
	return addr
}

func (chain *TestChain) InstantiateContract(suite *suite.Suite, msg string) sdk.AccAddress {
	quasarApp := chain.GetQuasarApp()
	contractKeeper := wasmkeeper.NewDefaultPermissionKeeper(quasarApp.WasmKeeper)
	codeID := uint64(1)
	creator := quasarApp.AccountKeeper.GetModuleAddress(govtypes.ModuleName)
	addr, _, err := contractKeeper.Instantiate(chain.GetContext(), codeID, creator, creator, []byte(msg), "contract", nil)
	suite.Require().NoError(err)
	return addr
}

func (chain *TestChain) QueryContract(suite *suite.Suite, contract sdk.AccAddress, key []byte) string {
	quasarApp := chain.GetQuasarApp()
	state, err := quasarApp.WasmKeeper.QuerySmart(chain.GetContext(), contract, key)
	suite.Require().NoError(err)
	return string(state)
}
