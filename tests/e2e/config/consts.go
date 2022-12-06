package config

import "time"

const (
	// DefaultNumValidators is the number of validator nodes deployed for each chain
	DefaultNumValidators = 1
	// DefaultNumNodes Number of full nodes deployed for each chain
	DefaultNumNodes = 0

	// VotingPeriod is the duration in which proposals in gov module are open for voting
	VotingPeriod = time.Second * 10

	// Default relayer path names for quasar <-> cosmos link
	Quasar2CosmosPath = "quasar-cosmos"
	// Default relayer path names for cosmos <-> osmosis link
	Cosmos2OsmosisPath = "cosmos-osmosis"
	// Default relayer path names for quasar <-> osmosis link
	Quasar2OsmosisPath = "quasar-osmosis"
)
