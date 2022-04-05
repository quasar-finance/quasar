# State

qbank manages the state of fund by using different combinations of prefixed keys.

## Prefixed Keys Value pairs

### Users current total deposit

Byte Prefix = types.UserDepositKBP =  []byte{0x04}
Key = Byte converted value of bech32 format user account = []byte(usersAccount)
Value = types.QCoins

Where:
usersAccount is the bech32 converted users account string
QCoins is defined as
type QCoins struct {
	Coins github_com_cosmos_cosmos_sdk_types.Coins `protobuf:"bytes,1,rep,name=coins,proto3,castrepeated=github.com/cosmos/cosmos-sdk/types.Coins" json:"coins"`
}

### Users current total deposit for a specific sdk.coin denom

Byte Prefix = types.UserDenomDepositKBP = []byte{0x02}
Key = byte converted value of {usersAccount} + ":" + {denom}
Value = object of type sdk.Coin

Where:
userAccound is the bech32 converted users account string
denom is the string value of the sdk.Coin token denomication.

### Users denom wise deposit on a specific epoch day for a specific lockup period

Byte Prefix = types.UserDenomDepositKBP = []byte{0x02}
Key = {epochday} + ":" + {lockupString} + ":" + {usersAccount} + ":" + {denom}
Value = object of type sdk.Coin

### Users current denom wise withdrawable amount

This KV pair stores the current value of withdrawable amount in sdk.Coin of any given user
Byte prefix = types.WithdrawableKeyKBP
Key = {denom} + ":" + {usersAccount}
Value = object of type sdk.Coin
