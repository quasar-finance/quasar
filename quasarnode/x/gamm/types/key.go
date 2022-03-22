package types

import "fmt"

const ModuleName = "quasar/gamm"

func GetPoolShareDenom(poolId uint64) string {
	return fmt.Sprintf("gamm/pool/%d", poolId)
}
