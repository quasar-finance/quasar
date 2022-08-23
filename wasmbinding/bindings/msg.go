package bindings

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

type QuasarMsg struct {
	/// Trigger test scenario
	TestScenario *TestScenario `json:"test_scenario,omitempty"`

	/// Contracts can send tokens
	SendToken *SendToken `json:"send_token,omitempty"`

	// Contracts can register interchain accounts
	RegisterInterchainAccount *RegisterInterchainAccount `json:"register_interchain_account,omitempty"`

	/// Contracts can transmit JoinPool Messages over IBC
	OsmosisJoinPool *OsmosisJoinPool `json:"join_pool,omitempty"`

	/// Contracts can transmit ExitPool Messages over IBC
	OsmosisExitPool *OsmosisExitPool `json:"exit_pool,omitempty"`

	/// Contracts can transmit LockTokens Messages over IBC
	OsmosisLockTokens *OsmosisLockTokens `json:"lock_tokens,omitempty"`

	/// Contracts can start the unbonding process over IBC
	OsmosisBeginUnlocking *OsmosisBeginUnlocking `json:"begin_unlocking,omitempty"`

	// Contracts can transmit JoinSwapExternAmountIn Messages over IBC
	OsmosisJoinSwapExternAmountIn *OsmosisJoinSwapExternAmountIn `json:"join_swap_extern_amount_in,omitempty"`

	// Contracts can transmit ExitSwapExternAmountOut Messages over IBC
	OsmosisExitSwapExternAmountOut *OsmosisExitSwapExternAmountOut `json:"exit_swap_extern_amount_out,omitempty"`
}

type TestScenario struct {
	Creator  string `json:"creator"`
	Scenario string `json:"scenario"`
}

type RegisterInterchainAccount struct {
	Creator string `json:"creator"`
	ConnectionId string `json:"connection_id"`
}

type SendToken struct {
	Creator                string   `json:"creator"`
	DestinationLocalZoneId string   `json:"destination_local_zone_id"`
	Sender                 string   `json:"sender"`
	Receiver               string   `json:"receiver"`
	Coin                   sdk.Coin `json:"coin"`
}

type OsmosisJoinPool struct {
	Creator          string     `json:"creator"`
	ConnectionId     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	PoolId           uint64     `json:"pool_id"`
	ShareOutAmount   int64      `json:"share_out_amount"`
	TokenInMaxs      []sdk.Coin `json:"token_in_maxs"`
}

type OsmosisExitPool struct {
	Creator          string     `json:"creator"`
	ConnectionId     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	PoolId           uint64     `json:"pool_id"`
	ShareInAmount    int64      `json:"share_in_amount"`
	TokenOutMins     []sdk.Coin `json:"token_out_mins"`
}

type OsmosisLockTokens struct {
	Creator          string     `json:"creator"`
	ConnectionId     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	Duration         uint64     `json:"duration"`
	Coins            []sdk.Coin `json:"coins"`
}

type OsmosisBeginUnlocking struct {
	Creator          string     `json:"creator"`
	ConnectionId     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	Id               uint64     `json:"id"`
	Coins            []sdk.Coin `json:"coins"`
}

type OsmosisJoinSwapExternAmountIn struct {
	Creator           string   `json:"creator"`
	ConnectionId      string   `json:"connection_id"`
	TimeoutTimestamp  uint64   `json:"timeout_timestamp"`
	PoolId            uint64   `json:"pool_id"`
	ShareOutMinAmount int64    `json:"share_out_min_amount"`
	TokenIn           sdk.Coin `json:"token_in"`
}

type OsmosisExitSwapExternAmountOut struct {
	Creator          string   `json:"creator"`
	ConnectionId     string   `json:"connection_id"`
	TimeoutTimestamp uint64   `json:"timeout_timestamp"`
	PoolId           uint64   `json:"pool_id"`
	ShareInAmount    int64    `json:"share_in_amount"`
	TokenOutMins     sdk.Coin `json:"token_out_mins"`
}
