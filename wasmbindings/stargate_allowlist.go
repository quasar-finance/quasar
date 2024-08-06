package wasmbindings

import (
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	icacontrollertypes "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/controller/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v8/modules/apps/transfer/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	ibcconnectiontypes "github.com/cosmos/ibc-go/v8/modules/core/03-connection/types"
	ibcchanneltypes "github.com/cosmos/ibc-go/v8/modules/core/04-channel/types"
	tokenfactorytypes "github.com/quasar-finance/quasar/x/tokenfactory/types"
)

func AcceptedStargateQueries() wasmkeeper.AcceptedQueries {
	return wasmkeeper.AcceptedQueries{
		//ibc
		"/ibc.core.client.v1.Query/ClientState":         &ibcclienttypes.QueryClientStateResponse{},
		"/ibc.core.client.v1.Query/ConsensusState":      &ibcclienttypes.QueryConsensusStateResponse{},
		"/ibc.core.connection.v1.Query/Connection":      &ibcconnectiontypes.QueryConnectionResponse{},
		"/ibc.core.channel.v1.Query/ChannelClientState": &ibcchanneltypes.QueryChannelClientStateResponse{},

		//token factory
		// token factory
		"/quasar.tokenfactory.v1beta1.Query/Params":                 &tokenfactorytypes.QueryParamsResponse{},
		"/quasar.tokenfactory.v1beta1.Query/DenomAuthorityMetadata": &tokenfactorytypes.QueryDenomAuthorityMetadataResponse{},
		"/quasar.tokenfactory.v1beta1.Query/DenomsFromCreator":      &tokenfactorytypes.QueryDenomsFromCreatorResponse{},
		"/quasar.tokenfactory.v1beta1.Query/BeforeSendHookAddress":  &tokenfactorytypes.QueryBeforeSendHookAddressResponse{},

		// interchain accounts
		"/ibc.applications.interchain_accounts.controller.v1.Query/InterchainAccount": &icacontrollertypes.QueryInterchainAccountResponse{},

		// transfer
		"/ibc.applications.transfer.v1.Query/DenomTrace": &ibctransfertypes.QueryDenomTraceResponse{},
	}
}
