// Code generated by MockGen. DO NOT EDIT.
// Source: github.com/quasarlabs/quasarnode/x/intergamm/types (interfaces: ConnectionKeeper)

// Package mock is a generated GoMock package.
package mock

import (
	reflect "reflect"

	types "github.com/cosmos/cosmos-sdk/types"
	types0 "github.com/cosmos/ibc-go/v6/modules/core/03-connection/types"
	gomock "github.com/golang/mock/gomock"
)

// MockConnectionKeeper is a mock of ConnectionKeeper interface.
type MockConnectionKeeper struct {
	ctrl     *gomock.Controller
	recorder *MockConnectionKeeperMockRecorder
}

// MockConnectionKeeperMockRecorder is the mock recorder for MockConnectionKeeper.
type MockConnectionKeeperMockRecorder struct {
	mock *MockConnectionKeeper
}

// NewMockConnectionKeeper creates a new mock instance.
func NewMockConnectionKeeper(ctrl *gomock.Controller) *MockConnectionKeeper {
	mock := &MockConnectionKeeper{ctrl: ctrl}
	mock.recorder = &MockConnectionKeeperMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use.
func (m *MockConnectionKeeper) EXPECT() *MockConnectionKeeperMockRecorder {
	return m.recorder
}

// GetAllConnections mocks base method.
func (m *MockConnectionKeeper) GetAllConnections(arg0 types.Context) []types0.IdentifiedConnection {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetAllConnections", arg0)
	ret0, _ := ret[0].([]types0.IdentifiedConnection)
	return ret0
}

// GetAllConnections indicates an expected call of GetAllConnections.
func (mr *MockConnectionKeeperMockRecorder) GetAllConnections(arg0 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetAllConnections", reflect.TypeOf((*MockConnectionKeeper)(nil).GetAllConnections), arg0)
}

// GetConnection mocks base method.
func (m *MockConnectionKeeper) GetConnection(arg0 types.Context, arg1 string) (types0.ConnectionEnd, bool) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetConnection", arg0, arg1)
	ret0, _ := ret[0].(types0.ConnectionEnd)
	ret1, _ := ret[1].(bool)
	return ret0, ret1
}

// GetConnection indicates an expected call of GetConnection.
func (mr *MockConnectionKeeperMockRecorder) GetConnection(arg0, arg1 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetConnection", reflect.TypeOf((*MockConnectionKeeper)(nil).GetConnection), arg0, arg1)
}
