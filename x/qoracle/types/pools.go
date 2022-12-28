package types

type PoolsOrderedByAPY []Pool

func (ops PoolsOrderedByAPY) Len() int {
	return len(ops)
}

func (ops PoolsOrderedByAPY) Less(i, j int) bool {
	return ops[i].APY.LT(ops[j].APY)
}

func (ops PoolsOrderedByAPY) Swap(i, j int) {
	ops[i], ops[j] = ops[j], ops[i]
}
