package bindings

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	bindingstypes "github.com/quasar-finance/quasar/x/tokenfactory/bindings/types"
)

// GetDenomAdmin is a query to get denom admin.
func (qp QueryPlugin) GetDenomAdmin(ctx sdk.Context, denom string) (*bindingstypes.AdminResponse, error) {
	metadata, err := qp.tokenFactoryKeeper.GetAuthorityMetadata(ctx, denom)
	if err != nil {
		return nil, fmt.Errorf("failed to get admin for denom: %s", denom)
	}
	return &bindingstypes.AdminResponse{Admin: metadata.Admin}, nil
}

// GetDenomsByCreator is a query to get list of denom strings created by a creator.
func (qp QueryPlugin) GetDenomsByCreator(ctx sdk.Context, creator string) (*bindingstypes.DenomsByCreatorResponse, error) {
	//validate creator address
	_, err := sdk.AccAddressFromBech32(creator)
	if err != nil {
		return nil, fmt.Errorf("invalid creator address: %s", creator)
	}
	denoms := qp.tokenFactoryKeeper.GetDenomsFromCreator(ctx, creator)
	return &bindingstypes.DenomsByCreatorResponse{Denoms: denoms}, nil
}

// GetMetadata is q query to get the stored metadata of a denom
func (qp QueryPlugin) GetMetadata(ctx sdk.Context, denom string) (*bindingstypes.MetadataResponse, error) {
	metadata, found := qp.bankKeeper.GetDenomMetaData(ctx, denom)
	var parsed *bindingstypes.Metadata
	if found {
		parsed = SdkMetadataToWasm(metadata)
	}
	return &bindingstypes.MetadataResponse{Metadata: parsed}, nil
}

func (qp QueryPlugin) GetParams(ctx sdk.Context) (*bindingstypes.ParamsResponse, error) {
	params := qp.tokenFactoryKeeper.GetParams(ctx)
	return &bindingstypes.ParamsResponse{
		Params: bindingstypes.Params{
			DenomCreationFee: ConvertSdkCoinsToWasmCoins(params.DenomCreationFee),
		},
	}, nil
}
