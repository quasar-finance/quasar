package testutils

import (
	"fmt"
	"os"

	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	transfertypes "github.com/cosmos/ibc-go/v7/modules/apps/transfer/types"
	"github.com/stretchr/testify/suite"
)

func (chain *TestChain) StoreContractCode(suite *suite.Suite, path string) {
	quasarApp := chain.GetQuasarApp()
	wasmCode, err := os.ReadFile(path)
	suite.Require().NoError(err)

	addr := quasarApp.AccountKeeper.GetModuleAddress(govtypes.ModuleName)
	msg := &wasmtypes.MsgStoreCode{
		Sender:       addr.String(),
		WASMByteCode: wasmCode,
		InstantiatePermission: &wasmtypes.AccessConfig{
			Permission: 3,
		},
	}
	handler := quasarApp.GovKeeper.Router().Handler(msg)

	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("handling x/gov proposal msg [%s] PANICKED: %v", msg, r)
		}
	}()
	_, err = handler(chain.GetContext(), msg)

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
