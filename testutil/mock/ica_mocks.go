// Code generated by MockGen. DO NOT EDIT.
// Source: github.com/quasarlabs/quasarnode/x/intergamm/types (interfaces: ICAControllerKeeper)

// Package mock is a generated GoMock package.
package mock

import (
	reflect "reflect"

	types "github.com/cosmos/cosmos-sdk/types"
	types0 "github.com/cosmos/ibc-go/modules/capability/types"
	types1 "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/types"
	gomock "github.com/golang/mock/gomock"
)

// MockICAControllerKeeper is a mock of ICAControllerKeeper interface.
type MockICAControllerKeeper struct {
	ctrl     *gomock.Controller
	recorder *MockICAControllerKeeperMockRecorder
}

// MockICAControllerKeeperMockRecorder is the mock recorder for MockICAControllerKeeper.
type MockICAControllerKeeperMockRecorder struct {
	mock *MockICAControllerKeeper
}

// NewMockICAControllerKeeper creates a new mock instance.
func NewMockICAControllerKeeper(ctrl *gomock.Controller) *MockICAControllerKeeper {
	mock := &MockICAControllerKeeper{ctrl: ctrl}
	mock.recorder = &MockICAControllerKeeperMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use.
func (m *MockICAControllerKeeper) EXPECT() *MockICAControllerKeeperMockRecorder {
	return m.recorder
}

// GetActiveChannelID mocks base method.
func (m *MockICAControllerKeeper) GetActiveChannelID(arg0 types.Context, arg1, arg2 string) (string, bool) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetActiveChannelID", arg0, arg1, arg2)
	ret0, _ := ret[0].(string)
	ret1, _ := ret[1].(bool)
	return ret0, ret1
}

// GetActiveChannelID indicates an expected call of GetActiveChannelID.
func (mr *MockICAControllerKeeperMockRecorder) GetActiveChannelID(arg0, arg1, arg2 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetActiveChannelID", reflect.TypeOf((*MockICAControllerKeeper)(nil).GetActiveChannelID), arg0, arg1, arg2)
}

// GetInterchainAccountAddress mocks base method.
func (m *MockICAControllerKeeper) GetInterchainAccountAddress(arg0 types.Context, arg1, arg2 string) (string, bool) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetInterchainAccountAddress", arg0, arg1, arg2)
	ret0, _ := ret[0].(string)
	ret1, _ := ret[1].(bool)
	return ret0, ret1
}

// GetInterchainAccountAddress indicates an expected call of GetInterchainAccountAddress.
func (mr *MockICAControllerKeeperMockRecorder) GetInterchainAccountAddress(arg0, arg1, arg2 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetInterchainAccountAddress", reflect.TypeOf((*MockICAControllerKeeper)(nil).GetInterchainAccountAddress), arg0, arg1, arg2)
}

// GetOpenActiveChannel mocks base method.
func (m *MockICAControllerKeeper) GetOpenActiveChannel(arg0 types.Context, arg1, arg2 string) (string, bool) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetOpenActiveChannel", arg0, arg1, arg2)
	ret0, _ := ret[0].(string)
	ret1, _ := ret[1].(bool)
	return ret0, ret1
}

// GetOpenActiveChannel indicates an expected call of GetOpenActiveChannel.
func (mr *MockICAControllerKeeperMockRecorder) GetOpenActiveChannel(arg0, arg1, arg2 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetOpenActiveChannel", reflect.TypeOf((*MockICAControllerKeeper)(nil).GetOpenActiveChannel), arg0, arg1, arg2)
}

// RegisterInterchainAccount mocks base method.
func (m *MockICAControllerKeeper) RegisterInterchainAccount(arg0 types.Context, arg1, arg2, arg3 string) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "RegisterInterchainAccount", arg0, arg1, arg2, arg3)
	ret0, _ := ret[0].(error)
	return ret0
}

// RegisterInterchainAccount indicates an expected call of RegisterInterchainAccount.
func (mr *MockICAControllerKeeperMockRecorder) RegisterInterchainAccount(arg0, arg1, arg2, arg3 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "RegisterInterchainAccount", reflect.TypeOf((*MockICAControllerKeeper)(nil).RegisterInterchainAccount), arg0, arg1, arg2, arg3)
}

// SendTx mocks base method.
func (m *MockICAControllerKeeper) SendTx(arg0 types.Context, arg1 *types0.Capability, arg2, arg3 string, arg4 types1.InterchainAccountPacketData, arg5 uint64) (uint64, error) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "SendTx", arg0, arg1, arg2, arg3, arg4, arg5)
	ret0, _ := ret[0].(uint64)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// SendTx indicates an expected call of SendTx.
func (mr *MockICAControllerKeeperMockRecorder) SendTx(arg0, arg1, arg2, arg3, arg4, arg5 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "SendTx", reflect.TypeOf((*MockICAControllerKeeper)(nil).SendTx), arg0, arg1, arg2, arg3, arg4, arg5)
}
