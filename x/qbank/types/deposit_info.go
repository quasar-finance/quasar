package types

// IsActiveOn determines if the DepositInfo is active on epochDay
func (m DepositInfo) IsActiveOn(epochDay uint64) bool {
	if epochDay < m.EpochDay {
		return false
	}
	switch m.LockupPeriod {
	case LockupTypes_Days_7:
		return epochDay <= m.EpochDay+7
	case LockupTypes_Days_21:
		return epochDay <= m.EpochDay+21
	case LockupTypes_Months_1:
		return epochDay <= m.EpochDay+31
	case LockupTypes_Months_3:
		return epochDay <= m.EpochDay+3*31
	}
	return false
}
