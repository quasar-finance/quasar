package balancer

import (
	"errors"
	time "time"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

// PoolI

func (p Pool) GetAddress() sdk.AccAddress                      { return nil }
func (p Pool) String() string                                  { return "" }
func (p Pool) GetId() uint64                                   { return 0 }
func (p Pool) GetSwapFee(ctx sdk.Context) sdk.Dec              { return sdk.ZeroDec() }
func (p Pool) GetExitFee(ctx sdk.Context) sdk.Dec              { return sdk.ZeroDec() }
func (p Pool) IsActive(ctx sdk.Context) bool                   { return false }
func (p Pool) GetTotalPoolLiquidity(ctx sdk.Context) sdk.Coins { return nil }
func (p Pool) GetTotalShares() sdk.Int                         { return sdk.ZeroInt() }

func (p Pool) SwapOutAmtGivenIn(ctx sdk.Context, tokenIn sdk.Coins, tokenOutDenom string, swapFee sdk.Dec) (tokenOut sdk.Coin, err error) {
	return sdk.Coin{}, errors.New("not implemented")
}

func (p Pool) CalcOutAmtGivenIn(ctx sdk.Context, tokenIn sdk.Coins, tokenOutDenom string, swapFee sdk.Dec) (tokenOut sdk.Coin, err error) {
	return sdk.Coin{}, errors.New("not implemented")
}

func (p Pool) SwapInAmtGivenOut(ctx sdk.Context, tokenOut sdk.Coins, tokenInDenom string, swapFee sdk.Dec) (tokenIn sdk.Coin, err error) {
	return sdk.Coin{}, errors.New("not implemented")
}

func (p Pool) CalcInAmtGivenOut(ctx sdk.Context, tokenOut sdk.Coins, tokenInDenom string, swapFee sdk.Dec) (tokenIn sdk.Coin, err error) {
	return sdk.Coin{}, errors.New("not implemented")
}

func (p Pool) SpotPrice(ctx sdk.Context, baseAssetDenom string, quoteAssetDenom string) (sdk.Dec, error) {
	return sdk.ZeroDec(), errors.New("not implemented")
}

func (p Pool) JoinPool(ctx sdk.Context, tokensIn sdk.Coins, swapFee sdk.Dec) (numShares sdk.Int, err error) {
	return sdk.ZeroInt(), errors.New("not implemented")
}

func (p Pool) CalcJoinPoolShares(ctx sdk.Context, tokensIn sdk.Coins, swapFee sdk.Dec) (numShares sdk.Int, newLiquidity sdk.Coins, err error) {
	return sdk.ZeroInt(), nil, errors.New("not implemented")
}

func (p Pool) ExitPool(ctx sdk.Context, numShares sdk.Int, exitFee sdk.Dec) (exitedCoins sdk.Coins, err error) {
	return nil, errors.New("not implemented")
}

func (p Pool) CalcExitPoolShares(ctx sdk.Context, numShares sdk.Int, exitFee sdk.Dec) (exitedCoins sdk.Coins, err error) {
	return nil, errors.New("not implemented")
}

func (p Pool) PokePool(blockTime time.Time) {}

// PoolAmountOutExtension

func (p Pool) CalcTokenInShareAmountOut(
	ctx sdk.Context,
	tokenInDenom string,
	shareOutAmount sdk.Int,
	swapFee sdk.Dec,
) (tokenInAmount sdk.Int, err error) {
	return sdk.ZeroInt(), errors.New("not implemented")
}

func (p Pool) JoinPoolTokenInMaxShareAmountOut(
	ctx sdk.Context,
	tokenInDenom string,
	shareOutAmount sdk.Int,
) (tokenInAmount sdk.Int, err error) {
	return sdk.ZeroInt(), errors.New("not implemented")
}

func (p Pool) ExitSwapExactAmountOut(
	ctx sdk.Context,
	tokenOut sdk.Coin,
	shareInMaxAmount sdk.Int,
) (shareInAmount sdk.Int, err error) {
	return sdk.ZeroInt(), errors.New("not implemented")
}

func (p Pool) IncreaseLiquidity(sharesOut sdk.Int, coinsIn sdk.Coins) {}
