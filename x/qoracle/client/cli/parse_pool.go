package cli

import (
	"encoding/json"
	"os"

	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
)

func parseBalancerPoolFile(poolFile string) (*gammbalancer.Pool, error) {
	contents, err := os.ReadFile(poolFile)
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
