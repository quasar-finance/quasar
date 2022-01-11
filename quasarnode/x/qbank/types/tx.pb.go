// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: qbank/tx.proto

package types

import (
	context "context"
	fmt "fmt"
	types "github.com/cosmos/cosmos-sdk/types"
	_ "github.com/gogo/protobuf/gogoproto"
	grpc1 "github.com/gogo/protobuf/grpc"
	proto "github.com/gogo/protobuf/proto"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	io "io"
	math "math"
	math_bits "math/bits"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.GoGoProtoPackageIsVersion3 // please upgrade the proto package

type MsgRequestDeposit struct {
	Creator     string     `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
	RiskProfile string     `protobuf:"bytes,2,opt,name=riskProfile,proto3" json:"riskProfile,omitempty"`
	VaultID     string     `protobuf:"bytes,3,opt,name=vaultID,proto3" json:"vaultID,omitempty"`
	Coin        types.Coin `protobuf:"bytes,4,opt,name=coin,proto3" json:"coin"`
}

func (m *MsgRequestDeposit) Reset()         { *m = MsgRequestDeposit{} }
func (m *MsgRequestDeposit) String() string { return proto.CompactTextString(m) }
func (*MsgRequestDeposit) ProtoMessage()    {}
func (*MsgRequestDeposit) Descriptor() ([]byte, []int) {
	return fileDescriptor_942456077037f563, []int{0}
}
func (m *MsgRequestDeposit) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgRequestDeposit) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgRequestDeposit.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgRequestDeposit) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgRequestDeposit.Merge(m, src)
}
func (m *MsgRequestDeposit) XXX_Size() int {
	return m.Size()
}
func (m *MsgRequestDeposit) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgRequestDeposit.DiscardUnknown(m)
}

var xxx_messageInfo_MsgRequestDeposit proto.InternalMessageInfo

func (m *MsgRequestDeposit) GetCreator() string {
	if m != nil {
		return m.Creator
	}
	return ""
}

func (m *MsgRequestDeposit) GetRiskProfile() string {
	if m != nil {
		return m.RiskProfile
	}
	return ""
}

func (m *MsgRequestDeposit) GetVaultID() string {
	if m != nil {
		return m.VaultID
	}
	return ""
}

func (m *MsgRequestDeposit) GetCoin() types.Coin {
	if m != nil {
		return m.Coin
	}
	return types.Coin{}
}

type MsgRequestDepositResponse struct {
}

func (m *MsgRequestDepositResponse) Reset()         { *m = MsgRequestDepositResponse{} }
func (m *MsgRequestDepositResponse) String() string { return proto.CompactTextString(m) }
func (*MsgRequestDepositResponse) ProtoMessage()    {}
func (*MsgRequestDepositResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_942456077037f563, []int{1}
}
func (m *MsgRequestDepositResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgRequestDepositResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgRequestDepositResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgRequestDepositResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgRequestDepositResponse.Merge(m, src)
}
func (m *MsgRequestDepositResponse) XXX_Size() int {
	return m.Size()
}
func (m *MsgRequestDepositResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgRequestDepositResponse.DiscardUnknown(m)
}

var xxx_messageInfo_MsgRequestDepositResponse proto.InternalMessageInfo

type MsgRequestWithdraw struct {
	Creator     string     `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
	RiskProfile string     `protobuf:"bytes,2,opt,name=riskProfile,proto3" json:"riskProfile,omitempty"`
	VaultID     string     `protobuf:"bytes,3,opt,name=vaultID,proto3" json:"vaultID,omitempty"`
	Coin        types.Coin `protobuf:"bytes,4,opt,name=coin,proto3" json:"coin"`
}

func (m *MsgRequestWithdraw) Reset()         { *m = MsgRequestWithdraw{} }
func (m *MsgRequestWithdraw) String() string { return proto.CompactTextString(m) }
func (*MsgRequestWithdraw) ProtoMessage()    {}
func (*MsgRequestWithdraw) Descriptor() ([]byte, []int) {
	return fileDescriptor_942456077037f563, []int{2}
}
func (m *MsgRequestWithdraw) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgRequestWithdraw) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgRequestWithdraw.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgRequestWithdraw) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgRequestWithdraw.Merge(m, src)
}
func (m *MsgRequestWithdraw) XXX_Size() int {
	return m.Size()
}
func (m *MsgRequestWithdraw) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgRequestWithdraw.DiscardUnknown(m)
}

var xxx_messageInfo_MsgRequestWithdraw proto.InternalMessageInfo

func (m *MsgRequestWithdraw) GetCreator() string {
	if m != nil {
		return m.Creator
	}
	return ""
}

func (m *MsgRequestWithdraw) GetRiskProfile() string {
	if m != nil {
		return m.RiskProfile
	}
	return ""
}

func (m *MsgRequestWithdraw) GetVaultID() string {
	if m != nil {
		return m.VaultID
	}
	return ""
}

func (m *MsgRequestWithdraw) GetCoin() types.Coin {
	if m != nil {
		return m.Coin
	}
	return types.Coin{}
}

type MsgRequestWithdrawResponse struct {
}

func (m *MsgRequestWithdrawResponse) Reset()         { *m = MsgRequestWithdrawResponse{} }
func (m *MsgRequestWithdrawResponse) String() string { return proto.CompactTextString(m) }
func (*MsgRequestWithdrawResponse) ProtoMessage()    {}
func (*MsgRequestWithdrawResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_942456077037f563, []int{3}
}
func (m *MsgRequestWithdrawResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgRequestWithdrawResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgRequestWithdrawResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgRequestWithdrawResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgRequestWithdrawResponse.Merge(m, src)
}
func (m *MsgRequestWithdrawResponse) XXX_Size() int {
	return m.Size()
}
func (m *MsgRequestWithdrawResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgRequestWithdrawResponse.DiscardUnknown(m)
}

var xxx_messageInfo_MsgRequestWithdrawResponse proto.InternalMessageInfo

func init() {
	proto.RegisterType((*MsgRequestDeposit)(nil), "abag.quasarnode.qbank.MsgRequestDeposit")
	proto.RegisterType((*MsgRequestDepositResponse)(nil), "abag.quasarnode.qbank.MsgRequestDepositResponse")
	proto.RegisterType((*MsgRequestWithdraw)(nil), "abag.quasarnode.qbank.MsgRequestWithdraw")
	proto.RegisterType((*MsgRequestWithdrawResponse)(nil), "abag.quasarnode.qbank.MsgRequestWithdrawResponse")
}

func init() { proto.RegisterFile("qbank/tx.proto", fileDescriptor_942456077037f563) }

var fileDescriptor_942456077037f563 = []byte{
	// 356 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xcc, 0x92, 0xcf, 0x4e, 0xfa, 0x40,
	0x10, 0xc7, 0xbb, 0x3f, 0xc8, 0xcf, 0xb8, 0x24, 0x18, 0x1b, 0x4d, 0x4a, 0x35, 0x95, 0x70, 0xaa,
	0x97, 0x5d, 0x81, 0x37, 0x40, 0x2e, 0x1e, 0x48, 0x4c, 0x2f, 0x26, 0xde, 0xb6, 0x65, 0x2d, 0x1b,
	0xa0, 0x53, 0x76, 0xb7, 0x88, 0x6f, 0xe1, 0x13, 0xe8, 0xeb, 0x70, 0xe4, 0xe8, 0xc9, 0x18, 0xf0,
	0x41, 0x4c, 0x5b, 0x1a, 0x0c, 0x98, 0xc8, 0xd1, 0xdb, 0xfc, 0xf9, 0xce, 0xcc, 0x27, 0x33, 0x83,
	0xab, 0x13, 0x9f, 0x45, 0x43, 0xaa, 0x67, 0x24, 0x96, 0xa0, 0xc1, 0x3c, 0x65, 0x3e, 0x0b, 0xc9,
	0x24, 0x61, 0x8a, 0xc9, 0x08, 0xfa, 0x9c, 0x64, 0x79, 0xdb, 0x09, 0x40, 0x8d, 0x41, 0x51, 0x9f,
	0x29, 0x4e, 0xa7, 0x4d, 0x9f, 0x6b, 0xd6, 0xa4, 0x01, 0x88, 0x28, 0x2f, 0xb3, 0x4f, 0x42, 0x08,
	0x21, 0x33, 0x69, 0x6a, 0xe5, 0xd1, 0xc6, 0x0b, 0xc2, 0xc7, 0x3d, 0x15, 0x7a, 0x7c, 0x92, 0x70,
	0xa5, 0xbb, 0x3c, 0x06, 0x25, 0xb4, 0x69, 0xe1, 0x83, 0x40, 0x72, 0xa6, 0x41, 0x5a, 0xa8, 0x8e,
	0xdc, 0x43, 0xaf, 0x70, 0xcd, 0x3a, 0xae, 0x48, 0xa1, 0x86, 0xb7, 0x12, 0x1e, 0xc4, 0x88, 0x5b,
	0xff, 0xb2, 0xec, 0xf7, 0x50, 0x5a, 0x3b, 0x65, 0xc9, 0x48, 0xdf, 0x74, 0xad, 0x52, 0x5e, 0xbb,
	0x76, 0xcd, 0x36, 0x2e, 0xa7, 0x3c, 0x56, 0xb9, 0x8e, 0xdc, 0x4a, 0xab, 0x46, 0x72, 0x60, 0x92,
	0x02, 0x93, 0x35, 0x30, 0xb9, 0x06, 0x11, 0x75, 0xca, 0xf3, 0xf7, 0x0b, 0xc3, 0xcb, 0xc4, 0x8d,
	0x33, 0x5c, 0xdb, 0xe1, 0xf3, 0xb8, 0x8a, 0x21, 0x52, 0xbc, 0xf1, 0x8a, 0xb0, 0xb9, 0xc9, 0xde,
	0x09, 0x3d, 0xe8, 0x4b, 0xf6, 0xf8, 0x97, 0xf0, 0xcf, 0xb1, 0xbd, 0x0b, 0x58, 0xf0, 0xb7, 0x3e,
	0x11, 0x2e, 0xf5, 0x54, 0x68, 0x8e, 0x70, 0x75, 0xeb, 0x02, 0x2e, 0xf9, 0xf1, 0xca, 0x64, 0x67,
	0x17, 0xf6, 0xd5, 0xbe, 0xca, 0x62, 0xaa, 0x09, 0xf8, 0x68, 0x7b, 0x63, 0x97, 0xbf, 0x36, 0x29,
	0xa4, 0x76, 0x73, 0x6f, 0x69, 0x31, 0xb0, 0xd3, 0x99, 0x2f, 0x1d, 0xb4, 0x58, 0x3a, 0xe8, 0x63,
	0xe9, 0xa0, 0xe7, 0x95, 0x63, 0x2c, 0x56, 0x8e, 0xf1, 0xb6, 0x72, 0x8c, 0x7b, 0x37, 0x14, 0x7a,
	0x90, 0xf8, 0x24, 0x80, 0x31, 0x4d, 0xdb, 0xd2, 0x4d, 0x5b, 0x3a, 0xa3, 0xeb, 0xc7, 0x7f, 0x8a,
	0xb9, 0xf2, 0xff, 0x67, 0xff, 0xda, 0xfe, 0x0a, 0x00, 0x00, 0xff, 0xff, 0xba, 0x95, 0x06, 0x15,
	0x0e, 0x03, 0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// MsgClient is the client API for Msg service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type MsgClient interface {
	RequestDeposit(ctx context.Context, in *MsgRequestDeposit, opts ...grpc.CallOption) (*MsgRequestDepositResponse, error)
	RequestWithdraw(ctx context.Context, in *MsgRequestWithdraw, opts ...grpc.CallOption) (*MsgRequestWithdrawResponse, error)
}

type msgClient struct {
	cc grpc1.ClientConn
}

func NewMsgClient(cc grpc1.ClientConn) MsgClient {
	return &msgClient{cc}
}

func (c *msgClient) RequestDeposit(ctx context.Context, in *MsgRequestDeposit, opts ...grpc.CallOption) (*MsgRequestDepositResponse, error) {
	out := new(MsgRequestDepositResponse)
	err := c.cc.Invoke(ctx, "/abag.quasarnode.qbank.Msg/RequestDeposit", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *msgClient) RequestWithdraw(ctx context.Context, in *MsgRequestWithdraw, opts ...grpc.CallOption) (*MsgRequestWithdrawResponse, error) {
	out := new(MsgRequestWithdrawResponse)
	err := c.cc.Invoke(ctx, "/abag.quasarnode.qbank.Msg/RequestWithdraw", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MsgServer is the server API for Msg service.
type MsgServer interface {
	RequestDeposit(context.Context, *MsgRequestDeposit) (*MsgRequestDepositResponse, error)
	RequestWithdraw(context.Context, *MsgRequestWithdraw) (*MsgRequestWithdrawResponse, error)
}

// UnimplementedMsgServer can be embedded to have forward compatible implementations.
type UnimplementedMsgServer struct {
}

func (*UnimplementedMsgServer) RequestDeposit(ctx context.Context, req *MsgRequestDeposit) (*MsgRequestDepositResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RequestDeposit not implemented")
}
func (*UnimplementedMsgServer) RequestWithdraw(ctx context.Context, req *MsgRequestWithdraw) (*MsgRequestWithdrawResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RequestWithdraw not implemented")
}

func RegisterMsgServer(s grpc1.Server, srv MsgServer) {
	s.RegisterService(&_Msg_serviceDesc, srv)
}

func _Msg_RequestDeposit_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(MsgRequestDeposit)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MsgServer).RequestDeposit(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/abag.quasarnode.qbank.Msg/RequestDeposit",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MsgServer).RequestDeposit(ctx, req.(*MsgRequestDeposit))
	}
	return interceptor(ctx, in, info, handler)
}

func _Msg_RequestWithdraw_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(MsgRequestWithdraw)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MsgServer).RequestWithdraw(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/abag.quasarnode.qbank.Msg/RequestWithdraw",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MsgServer).RequestWithdraw(ctx, req.(*MsgRequestWithdraw))
	}
	return interceptor(ctx, in, info, handler)
}

var _Msg_serviceDesc = grpc.ServiceDesc{
	ServiceName: "abag.quasarnode.qbank.Msg",
	HandlerType: (*MsgServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "RequestDeposit",
			Handler:    _Msg_RequestDeposit_Handler,
		},
		{
			MethodName: "RequestWithdraw",
			Handler:    _Msg_RequestWithdraw_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "qbank/tx.proto",
}

func (m *MsgRequestDeposit) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgRequestDeposit) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgRequestDeposit) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	{
		size, err := m.Coin.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintTx(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x22
	if len(m.VaultID) > 0 {
		i -= len(m.VaultID)
		copy(dAtA[i:], m.VaultID)
		i = encodeVarintTx(dAtA, i, uint64(len(m.VaultID)))
		i--
		dAtA[i] = 0x1a
	}
	if len(m.RiskProfile) > 0 {
		i -= len(m.RiskProfile)
		copy(dAtA[i:], m.RiskProfile)
		i = encodeVarintTx(dAtA, i, uint64(len(m.RiskProfile)))
		i--
		dAtA[i] = 0x12
	}
	if len(m.Creator) > 0 {
		i -= len(m.Creator)
		copy(dAtA[i:], m.Creator)
		i = encodeVarintTx(dAtA, i, uint64(len(m.Creator)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *MsgRequestDepositResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgRequestDepositResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgRequestDepositResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	return len(dAtA) - i, nil
}

func (m *MsgRequestWithdraw) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgRequestWithdraw) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgRequestWithdraw) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	{
		size, err := m.Coin.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintTx(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x22
	if len(m.VaultID) > 0 {
		i -= len(m.VaultID)
		copy(dAtA[i:], m.VaultID)
		i = encodeVarintTx(dAtA, i, uint64(len(m.VaultID)))
		i--
		dAtA[i] = 0x1a
	}
	if len(m.RiskProfile) > 0 {
		i -= len(m.RiskProfile)
		copy(dAtA[i:], m.RiskProfile)
		i = encodeVarintTx(dAtA, i, uint64(len(m.RiskProfile)))
		i--
		dAtA[i] = 0x12
	}
	if len(m.Creator) > 0 {
		i -= len(m.Creator)
		copy(dAtA[i:], m.Creator)
		i = encodeVarintTx(dAtA, i, uint64(len(m.Creator)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *MsgRequestWithdrawResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgRequestWithdrawResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgRequestWithdrawResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	return len(dAtA) - i, nil
}

func encodeVarintTx(dAtA []byte, offset int, v uint64) int {
	offset -= sovTx(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *MsgRequestDeposit) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.Creator)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	l = len(m.RiskProfile)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	l = len(m.VaultID)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	l = m.Coin.Size()
	n += 1 + l + sovTx(uint64(l))
	return n
}

func (m *MsgRequestDepositResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	return n
}

func (m *MsgRequestWithdraw) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.Creator)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	l = len(m.RiskProfile)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	l = len(m.VaultID)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	l = m.Coin.Size()
	n += 1 + l + sovTx(uint64(l))
	return n
}

func (m *MsgRequestWithdrawResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	return n
}

func sovTx(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozTx(x uint64) (n int) {
	return sovTx(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *MsgRequestDeposit) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowTx
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: MsgRequestDeposit: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgRequestDeposit: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Creator", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Creator = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field RiskProfile", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.RiskProfile = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field VaultID", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.VaultID = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Coin", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.Coin.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipTx(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthTx
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *MsgRequestDepositResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowTx
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: MsgRequestDepositResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgRequestDepositResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		default:
			iNdEx = preIndex
			skippy, err := skipTx(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthTx
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *MsgRequestWithdraw) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowTx
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: MsgRequestWithdraw: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgRequestWithdraw: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Creator", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Creator = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field RiskProfile", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.RiskProfile = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field VaultID", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.VaultID = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Coin", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthTx
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthTx
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.Coin.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipTx(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthTx
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *MsgRequestWithdrawResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowTx
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: MsgRequestWithdrawResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgRequestWithdrawResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		default:
			iNdEx = preIndex
			skippy, err := skipTx(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthTx
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipTx(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowTx
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowTx
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowTx
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthTx
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupTx
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthTx
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthTx        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowTx          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupTx = fmt.Errorf("proto: unexpected end of group")
)
