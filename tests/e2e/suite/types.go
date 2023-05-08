package suite

import (
	sdkTypes "github.com/cosmos/cosmos-sdk/types"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
)

type Accounts struct {
	Treasury ibc.Wallet
}

type Balance struct {
	Balance string `json:"balance"`
}

type ContractBalanceData struct {
	Data Balance `json:"data"`
}

type PendingUnbondsData struct {
	Data Unbonds `json:"data"`
}

type Unbonds struct {
	PendingUnbonds   []PendingUnbonds `json:"pending_unbonds"`
	PendingUnbondIds []string         `json:"pending_unbond_ids"`
}

type PendingUnbonds struct {
	Stub   []UnbondDetails `json:"stub"`
	Shares string          `json:"shares"`
}

type UnbondDetails struct {
	Address        string        `json:"address"`
	UnlockTime     string        `json:"unlock_time"`
	UnbondResponse interface{}   `json:"unbond_response"`
	UnbondFunds    []interface{} `json:"unbond_funds"`
}

type QueryAllBalancesResponse struct {
	Balances   sdkTypes.Coins `json:"balances"`
	Pagination PageResponse   `json:"pagination,omitempty"`
}

type PageResponse struct {
	NextKey []byte `protobuf:"bytes,1,opt,name=next_key,json=nextKey,proto3" json:"next_key,omitempty"`
	Total   string `protobuf:"varint,2,opt,name=total,proto3" json:"total,omitempty"`
}
