package cli

import (
	"encoding/json"
	"io/ioutil"

	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
)

func parseBalancerPoolFile(poolFile string) (*gammbalancer.Pool, error) {
	contents, err := ioutil.ReadFile(poolFile)
	if err != nil {
		return nil, err
	}

	pool := &gammbalancer.Pool{}
	err = json.Unmarshal(contents, pool)
	if err != nil {
		return nil, err
	}

	return pool, nil
}
