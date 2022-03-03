
# Messages 

qbank support following messages for users activity in the quasar. 

## Request deposit transaction message
message MsgRequestDeposit {
  string creator = 1;
  string riskProfile = 2;
  string vaultID = 3;
  cosmos.base.v1beta1.Coin coin = 4 [ (gogoproto.nullable) = false ];
  // string lockupPeriod = 5;
  LockupTypes lockupPeriod  = 5;
}

## Request Withdraw transaction message is used to request withdraw a particular token 

message MsgRequestWithdraw {
  string creator = 1;
  string riskProfile = 2;
  string vaultID = 3;
  cosmos.base.v1beta1.Coin coin = 4 [ (gogoproto.nullable) = false ];
}

 ## Claim rewards transaction message

message MsgClaimRewards {
  string creator = 1;
  string vaultID = 2;
}