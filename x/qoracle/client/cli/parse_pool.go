package cli

import (
	"encoding/json"
	"io/ioutil"

	"github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

func parseBalancerPoolFile(poolFile string) (*balancer.BalancerPool, error) {
	contents, err := ioutil.ReadFile(poolFile)
	if err != nil {
		return nil, err
	}

	pool := &balancer.BalancerPool{}
	err = json.Unmarshal(contents, pool)
	if err != nil {
		return nil, err
	}

	return pool, nil
}
