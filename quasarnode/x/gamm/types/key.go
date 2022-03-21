package types

import "fmt"

const ModuleName = "gamm"

func GetPoolShareDenom(poolId uint64) string {
	return fmt.Sprintf("gamm/pool/%d", poolId)
}
