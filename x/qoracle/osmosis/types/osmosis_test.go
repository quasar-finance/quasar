package types

import (
	"testing"
	"time"

	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/pool-incentives/types"
	"github.com/stretchr/testify/assert"
)

func TestUniquePoolIdsFromIncentivizedPools(t *testing.T) {
	incentivizedPools := []poolincentivestypes.IncentivizedPool{
		{
			PoolId:           1,
			GaugeId:          1,
			LockableDuration: time.Hour,
		},
		{
			PoolId:           1,
			GaugeId:          2,
			LockableDuration: time.Hour * 24,
		},
		{
			PoolId:           3,
			GaugeId:          4,
			LockableDuration: time.Minute,
		},
		{
			PoolId:           3,
			GaugeId:          3,
			LockableDuration: time.Minute * 5,
		},
		{
			PoolId:           4,
			GaugeId:          7,
			LockableDuration: time.Hour * 24 * 7,
		},
	}
	poolIds := UniquePoolIdsFromIncentivizedPools(incentivizedPools)
	assert.Equal(t, poolIds, []uint64{1, 3, 4})
}
