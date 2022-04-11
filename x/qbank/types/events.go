package types

import (
	"strconv"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

const (
	TypeEvtDeposit      = "deposit"
	TypeEvtWithdraw     = "withdraw"
	TypeEvtWithdrawAll  = "withdraw_all"
	TypeEvtClaimRewards = "claim_rewards"

	AttributeValueCategory          = ModuleName
	AttributeKeyDepositCoin         = "deposit_coin"
	AttributeKeyDepositLockupPeriod = "deposit_lockup_period"
	AttributeKeyDepositEpoch        = "deposit_epoch"
	AttributeKeyWithdrawCoin        = "withdraw_coin"
	AttributeKeyWithdrawVaultId     = "withdraw_vault_id"
	AttributeKeyWithdrawRiskProfile = "withdraw_risk_profile"
	AttributeKeyWithdrawAllVaultId  = "withdraw_all_vault_id"
	AttributeKeyClaimRewardsVaultId = "claim_rewards_vault_id"
)

func CreateDepositEvent(ctx sdk.Context, sender sdk.AccAddress, coin sdk.Coin, lockupPeriod LockupTypes, currentEpoch uint64) sdk.Event {
	return sdk.NewEvent(
		TypeEvtDeposit,
		sdk.NewAttribute(sdk.AttributeKeyModule, AttributeValueCategory),
		sdk.NewAttribute(sdk.AttributeKeySender, sender.String()),
		sdk.NewAttribute(AttributeKeyDepositCoin, coin.String()),
		sdk.NewAttribute(AttributeKeyDepositLockupPeriod, lockupPeriod.String()),
		sdk.NewAttribute(AttributeKeyDepositEpoch, strconv.FormatUint(currentEpoch, 10)),
	)
}

func CreateWithdrawEvent(ctx sdk.Context, sender sdk.AccAddress, coin sdk.Coin, vaultId string, riskProfile string) sdk.Event {
	return sdk.NewEvent(
		TypeEvtWithdraw,
		sdk.NewAttribute(sdk.AttributeKeyModule, AttributeValueCategory),
		sdk.NewAttribute(sdk.AttributeKeySender, sender.String()),
		sdk.NewAttribute(AttributeKeyWithdrawCoin, coin.String()),
		sdk.NewAttribute(AttributeKeyWithdrawVaultId, vaultId),
		sdk.NewAttribute(AttributeKeyWithdrawRiskProfile, riskProfile),
	)
}

func CreateWithdrawAllEvent(ctx sdk.Context, sender sdk.AccAddress, vaultId string) sdk.Event {
	return sdk.NewEvent(
		TypeEvtWithdrawAll,
		sdk.NewAttribute(sdk.AttributeKeyModule, AttributeValueCategory),
		sdk.NewAttribute(sdk.AttributeKeySender, sender.String()),
		sdk.NewAttribute(AttributeKeyWithdrawAllVaultId, vaultId),
	)
}

func CreateClaimRewardsEvent(ctx sdk.Context, sender sdk.AccAddress, vaultId string) sdk.Event {
	return sdk.NewEvent(
		TypeEvtClaimRewards,
		sdk.NewAttribute(sdk.AttributeKeyModule, AttributeValueCategory),
		sdk.NewAttribute(sdk.AttributeKeySender, sender.String()),
		sdk.NewAttribute(AttributeKeyClaimRewardsVaultId, vaultId),
	)
}
