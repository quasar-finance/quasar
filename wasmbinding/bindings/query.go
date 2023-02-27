package bindings

// QuasarQuery contains quasar custom queries.
type QuasarQuery struct {
	// Query all pools
	PoolsRankedByAPY *PoolsRankedByAPYRequest `json:"pools_ranked_by_apy,omitempty"`

	// Query pool details
	Pool *PoolRequest `json:"pool,omitempty"`

	// Query token price
	TokenPrice *TokenPriceRequest `json:"token_price,omitempty"`
}

type PoolsRankedByAPYRequest struct {
	Denom string `json:"denom"`
}

type PoolRequest struct {
	Id string `json:"id"`
}

type TokenPriceRequest struct {
	Denom string `json:"denom"`
}
