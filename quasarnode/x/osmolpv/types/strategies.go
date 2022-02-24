package types

import (
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
)

// TODO | AUDIT | Can/should these be managed in module parameter / genesis parameter
// Can/should these be a field in strategy object
const (
	MeissaInvalid = "Invalid"
	Meissa7d      = "Meissa.7d"
	Meissa21d     = "Meissa.21d"
	Meissa1m      = "Meissa.1m"
	Meissa3m      = "Meissa.3m"
)

var MeissaStrategiesLockup = map[string]qbanktypes.LockupTypes{
	MeissaInvalid: qbanktypes.LockupTypes_Invalid,
	Meissa7d:      qbanktypes.LockupTypes_Days_7,
	Meissa21d:     qbanktypes.LockupTypes_Days_21,
	Meissa1m:      qbanktypes.LockupTypes_Months_1,
	Meissa3m:      qbanktypes.LockupTypes_Months_3,
	/*
		"Invalid":    qbanktypes.LockupTypes_Invalid,
		"Meissa.7d":  qbanktypes.LockupTypes_Days_7,
		"Meissa.21d": qbanktypes.LockupTypes_Days_21,
		"Meissa.1m":  qbanktypes.LockupTypes_Months_1,
		"Meissa.3m":  qbanktypes.LockupTypes_Months_3,
	*/
	//"Meissa.6m":  banktypes.LockupTypes_Months_6,
	//"Meissa.9m":  banktypes.LockupTypes_Months_9,
	//"Meissa.12m":  banktypes.LockupTypes_Months_12,
}

var LockupMeissaStrategies = map[qbanktypes.LockupTypes]string{

	qbanktypes.LockupTypes_Invalid:  MeissaInvalid,
	qbanktypes.LockupTypes_Days_7:   Meissa7d,
	qbanktypes.LockupTypes_Days_21:  Meissa21d,
	qbanktypes.LockupTypes_Months_1: Meissa1m,
	qbanktypes.LockupTypes_Months_3: Meissa3m,
}
