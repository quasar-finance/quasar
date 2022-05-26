package types

import (
	"github.com/stretchr/testify/require"
	"testing"
)

func TestDepositInfo_IsActiveOn(t *testing.T) {
	var tests = []struct {
		name          string
		startEpochDay uint64
		lockupPeriod  LockupTypes
		todayEpochDay uint64
		isActive      bool
	}{
		{
			name:          "before start",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Days_7,
			todayEpochDay: 1,
			isActive:      false,
		},
		{
			name:          "before start",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Days_7,
			todayEpochDay: 1,
			isActive:      false,
		},
		{
			name:          "invalid lockup period",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Invalid,
			todayEpochDay: 3,
			isActive:      false,
		},
		{
			name:          "active 1-week",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Days_7,
			todayEpochDay: 9,
			isActive:      true,
		},
		{
			name:          "after end (1-week)",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Days_7,
			todayEpochDay: 10,
			isActive:      false,
		},
		{
			name:          "active 3-week",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Days_21,
			todayEpochDay: 15,
			isActive:      true,
		},
		{
			name:          "active 3-week",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Days_21,
			todayEpochDay: 23,
			isActive:      true,
		},
		{
			name:          "after end (3-week)",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Days_7,
			todayEpochDay: 24,
			isActive:      false,
		},
		{
			name:          "active 1-month",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Months_1,
			todayEpochDay: 15,
			isActive:      true,
		},
		{
			name:          "active 1-month",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Months_1,
			todayEpochDay: 32,
			isActive:      true,
		},
		{
			name:          "after end (1-month)",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Months_1,
			todayEpochDay: 34,
			isActive:      false,
		},
		{
			name:          "active 3-month",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Months_3,
			todayEpochDay: 15,
			isActive:      true,
		},
		{
			name:          "active 3-month",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Months_3,
			todayEpochDay: 30,
			isActive:      true,
		},
		{
			name:          "after end (3-month)",
			startEpochDay: 2,
			lockupPeriod:  LockupTypes_Months_3,
			todayEpochDay: 97,
			isActive:      false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			depositInfo := DepositInfo{
				EpochDay:     tt.startEpochDay,
				LockupPeriod: tt.lockupPeriod,
			}
			isActive := depositInfo.IsActiveOn(tt.todayEpochDay)
			require.Equal(t, tt.isActive, isActive)
		})
	}
}
