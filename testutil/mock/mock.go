package mock

import (
	"testing"

	"github.com/golang/mock/gomock"
)

type TestMocks struct {
	ICAControllerKeeper *MockICAControllerKeeper
}

func NewTestMocks(t *testing.T, ctl *gomock.Controller) *TestMocks {
	return &TestMocks{
		ICAControllerKeeper: NewMockICAControllerKeeper(ctl),
	}
}
