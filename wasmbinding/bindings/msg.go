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
	RegisterICAOnZone *RegisterICAOnZone `json:"register_ica_on_zone,omitempty"`

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
	Scenario string `json:"scenario"`
}

type RegisterICAOnZone struct {
	ZoneID string `json:"zone_id"`
}

type SendToken struct {
	DestinationLocalZoneID string   `json:"destination_local_zone_id"`
	Receiver               string   `json:"receiver"`
	Coin                   sdk.Coin `json:"coin"`
}

type OsmosisJoinPool struct {
	ConnectionID     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	PoolID           uint64     `json:"pool_id"`
	ShareOutAmount   int64      `json:"share_out_amount"`
	TokenInMaxs      []sdk.Coin `json:"token_in_maxs"`
}

type OsmosisExitPool struct {
	ConnectionID     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	PoolID           uint64     `json:"pool_id"`
	ShareInAmount    int64      `json:"share_in_amount"`
	TokenOutMins     []sdk.Coin `json:"token_out_mins"`
}

type OsmosisLockTokens struct {
	ConnectionID     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	Duration         uint64     `json:"duration"`
	Coins            []sdk.Coin `json:"coins"`
}

type OsmosisBeginUnlocking struct {
	ConnectionID     string     `json:"connection_id"`
	TimeoutTimestamp uint64     `json:"timeout_timestamp"`
	ID               uint64     `json:"id"`
	Coins            []sdk.Coin `json:"coins"`
}

type OsmosisJoinSwapExternAmountIn struct {
	ConnectionID      string   `json:"connection_id"`
	TimeoutTimestamp  uint64   `json:"timeout_timestamp"`
	PoolID            uint64   `json:"pool_id"`
	ShareOutMinAmount int64    `json:"share_out_min_amount"`
	TokenIn           sdk.Coin `json:"token_in"`
}

type OsmosisExitSwapExternAmountOut struct {
	ConnectionID     string   `json:"connection_id"`
	TimeoutTimestamp uint64   `json:"timeout_timestamp"`
	PoolID           uint64   `json:"pool_id"`
	ShareInAmount    int64    `json:"share_in_amount"`
	TokenOutMins     sdk.Coin `json:"token_out_mins"`
}
