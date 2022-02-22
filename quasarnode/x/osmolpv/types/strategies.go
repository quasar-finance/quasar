package types

import (
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
)

var MeissaStrategiesLockup = map[string]qbanktypes.LockupTypes{
	"Invalid":    qbanktypes.LockupTypes_Invalid,
	"Meissa.7d":  qbanktypes.LockupTypes_Days_7,
	"Meissa.21d": qbanktypes.LockupTypes_Days_21,
	"Meissa.1m":  qbanktypes.LockupTypes_Months_1,
	"Meissa.3m":  qbanktypes.LockupTypes_Months_3,
	//"Meissa.6m":  banktypes.LockupTypes_Months_6,
	//"Meissa.9m":  banktypes.LockupTypes_Months_9,
	//"Meissa.12m":  banktypes.LockupTypes_Months_12,
}
