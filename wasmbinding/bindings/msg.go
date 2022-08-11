package bindings

import sdk "github.com/cosmos/cosmos-sdk/types"

type QuasarMsg struct {
	/// Trigger test scenario
	TestScenario *TestScenario `json:"test_scenario,omitempty"`

	/// Contracts can send tokens
	SendToken *SendToken `json:"send_token,omitempty"`
}

type TestScenario struct {
	Creator  string `json:"creator"`
	Scenario string `json:"scenario"`
}

type SendToken struct {
	Creator                string   `json:"creator"`
	DestinationLocalZoneId string   `json:"destination_local_zone_id"`
	Sender                 string   `json:"sender"`
	Receiver               string   `json:"receiver"`
	Coin                   sdk.Coin `json:"coin"`
}
