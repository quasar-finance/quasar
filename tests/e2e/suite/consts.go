package suite

import "time"

const (
	// DefaultNumValidators is the number of validator nodes deployed for each chain
	DefaultNumValidators = 1
	// DefaultNumNodes Number of full nodes deployed for each chain
	DefaultNumNodes = 0

	// VotingPeriod is the duration in which proposals in gov module are open for voting
	VotingPeriod = time.Second * 10

	// Default Relayer path names for quasar <-> cosmos link
	Quasar2CosmosPath = "quasar-cosmos"
	// Default Relayer path names for cosmos <-> osmosis link
	Cosmos2OsmosisPath = "cosmos-osmosis"
	// Default Relayer path names for quasar <-> osmosis link
	Quasar2OsmosisPath = "quasar-osmosis"
)

const (
	authorityKeyName = "authority"

	ownerKeyName        = "owner"
	ownerKeyName1       = "pppppppppppppp"
	newOwnerKeyName     = "new_owner"
	masterMinterKeyName = "masterminter"
	bondTestKeyName     = "bond_test"
	bondTestKeyName1    = "bond_test_1"
	bondTestKeyName2    = "bond_test_2"
	bondTestKeyName3    = "bond_test_3"
	bondTestKeyName4    = "bond_test_4"
	bondTestKeyName5    = "bond_test_5"
	bondTestKeyName6    = "bond_test_6"
	bondTestKeyName7    = "bond_test_7"
)
