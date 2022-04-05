package cli

import (
	"encoding/json"
	"io/ioutil"

	"github.com/abag/quasarnode/x/gamm/pool-models/balancer"
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
