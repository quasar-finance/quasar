// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: osmosis/v9/incentives/tx.proto

package types

import (
	context "context"
	fmt "fmt"
	types "github.com/abag/quasarnode/osmosis/v9/lockup/types"
	github_com_cosmos_cosmos_sdk_types "github.com/cosmos/cosmos-sdk/types"
	types1 "github.com/cosmos/cosmos-sdk/types"
	_ "github.com/gogo/protobuf/gogoproto"
	grpc1 "github.com/gogo/protobuf/grpc"
	proto "github.com/gogo/protobuf/proto"
	github_com_gogo_protobuf_types "github.com/gogo/protobuf/types"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	_ "google.golang.org/protobuf/types/known/timestamppb"
	io "io"
	math "math"
	math_bits "math/bits"
	time "time"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf
var _ = time.Kitchen

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.GoGoProtoPackageIsVersion3 // please upgrade the proto package

type MsgCreateGauge struct {
	// flag to show if it's perpetual or multi-epoch
	// distribution incentives by third party
	IsPerpetual bool   `protobuf:"varint,1,opt,name=is_perpetual,json=isPerpetual,proto3" json:"is_perpetual,omitempty"`
	Owner       string `protobuf:"bytes,2,opt,name=owner,proto3" json:"owner,omitempty" yaml:"owner"`
	// distribute condition of a lock which meet one of these conditions
	DistributeTo types.QueryCondition `protobuf:"bytes,3,opt,name=distribute_to,json=distributeTo,proto3" json:"distribute_to"`
	// can distribute multiple coins
	Coins github_com_cosmos_cosmos_sdk_types.Coins `protobuf:"bytes,4,rep,name=coins,proto3,castrepeated=github.com/cosmos/cosmos-sdk/types.Coins" json:"coins"`
	// distribution start time
	StartTime time.Time `protobuf:"bytes,5,opt,name=start_time,json=startTime,proto3,stdtime" json:"start_time" yaml:"timestamp"`
	// number of epochs distribution will be done
	NumEpochsPaidOver uint64 `protobuf:"varint,6,opt,name=num_epochs_paid_over,json=numEpochsPaidOver,proto3" json:"num_epochs_paid_over,omitempty"`
}

func (m *MsgCreateGauge) Reset()         { *m = MsgCreateGauge{} }
func (m *MsgCreateGauge) String() string { return proto.CompactTextString(m) }
func (*MsgCreateGauge) ProtoMessage()    {}
func (*MsgCreateGauge) Descriptor() ([]byte, []int) {
	return fileDescriptor_c759ef67f8072133, []int{0}
}
func (m *MsgCreateGauge) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgCreateGauge) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgCreateGauge.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgCreateGauge) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgCreateGauge.Merge(m, src)
}
func (m *MsgCreateGauge) XXX_Size() int {
	return m.Size()
}
func (m *MsgCreateGauge) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgCreateGauge.DiscardUnknown(m)
}

var xxx_messageInfo_MsgCreateGauge proto.InternalMessageInfo

func (m *MsgCreateGauge) GetIsPerpetual() bool {
	if m != nil {
		return m.IsPerpetual
	}
	return false
}

func (m *MsgCreateGauge) GetOwner() string {
	if m != nil {
		return m.Owner
	}
	return ""
}

func (m *MsgCreateGauge) GetDistributeTo() types.QueryCondition {
	if m != nil {
		return m.DistributeTo
	}
	return types.QueryCondition{}
}

func (m *MsgCreateGauge) GetCoins() github_com_cosmos_cosmos_sdk_types.Coins {
	if m != nil {
		return m.Coins
	}
	return nil
}

func (m *MsgCreateGauge) GetStartTime() time.Time {
	if m != nil {
		return m.StartTime
	}
	return time.Time{}
}

func (m *MsgCreateGauge) GetNumEpochsPaidOver() uint64 {
	if m != nil {
		return m.NumEpochsPaidOver
	}
	return 0
}

type MsgCreateGaugeResponse struct {
}

func (m *MsgCreateGaugeResponse) Reset()         { *m = MsgCreateGaugeResponse{} }
func (m *MsgCreateGaugeResponse) String() string { return proto.CompactTextString(m) }
func (*MsgCreateGaugeResponse) ProtoMessage()    {}
func (*MsgCreateGaugeResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_c759ef67f8072133, []int{1}
}
func (m *MsgCreateGaugeResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgCreateGaugeResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgCreateGaugeResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgCreateGaugeResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgCreateGaugeResponse.Merge(m, src)
}
func (m *MsgCreateGaugeResponse) XXX_Size() int {
	return m.Size()
}
func (m *MsgCreateGaugeResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgCreateGaugeResponse.DiscardUnknown(m)
}

var xxx_messageInfo_MsgCreateGaugeResponse proto.InternalMessageInfo

type MsgAddToGauge struct {
	Owner   string                                   `protobuf:"bytes,1,opt,name=owner,proto3" json:"owner,omitempty" yaml:"owner"`
	GaugeId uint64                                   `protobuf:"varint,2,opt,name=gauge_id,json=gaugeId,proto3" json:"gauge_id,omitempty"`
	Rewards github_com_cosmos_cosmos_sdk_types.Coins `protobuf:"bytes,3,rep,name=rewards,proto3,castrepeated=github.com/cosmos/cosmos-sdk/types.Coins" json:"rewards"`
}

func (m *MsgAddToGauge) Reset()         { *m = MsgAddToGauge{} }
func (m *MsgAddToGauge) String() string { return proto.CompactTextString(m) }
func (*MsgAddToGauge) ProtoMessage()    {}
func (*MsgAddToGauge) Descriptor() ([]byte, []int) {
	return fileDescriptor_c759ef67f8072133, []int{2}
}
func (m *MsgAddToGauge) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgAddToGauge) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgAddToGauge.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgAddToGauge) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgAddToGauge.Merge(m, src)
}
func (m *MsgAddToGauge) XXX_Size() int {
	return m.Size()
}
func (m *MsgAddToGauge) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgAddToGauge.DiscardUnknown(m)
}

var xxx_messageInfo_MsgAddToGauge proto.InternalMessageInfo

func (m *MsgAddToGauge) GetOwner() string {
	if m != nil {
		return m.Owner
	}
	return ""
}

func (m *MsgAddToGauge) GetGaugeId() uint64 {
	if m != nil {
		return m.GaugeId
	}
	return 0
}

func (m *MsgAddToGauge) GetRewards() github_com_cosmos_cosmos_sdk_types.Coins {
	if m != nil {
		return m.Rewards
	}
	return nil
}

type MsgAddToGaugeResponse struct {
}

func (m *MsgAddToGaugeResponse) Reset()         { *m = MsgAddToGaugeResponse{} }
func (m *MsgAddToGaugeResponse) String() string { return proto.CompactTextString(m) }
func (*MsgAddToGaugeResponse) ProtoMessage()    {}
func (*MsgAddToGaugeResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_c759ef67f8072133, []int{3}
}
func (m *MsgAddToGaugeResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *MsgAddToGaugeResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_MsgAddToGaugeResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *MsgAddToGaugeResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MsgAddToGaugeResponse.Merge(m, src)
}
func (m *MsgAddToGaugeResponse) XXX_Size() int {
	return m.Size()
}
func (m *MsgAddToGaugeResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_MsgAddToGaugeResponse.DiscardUnknown(m)
}

var xxx_messageInfo_MsgAddToGaugeResponse proto.InternalMessageInfo

func init() {
	proto.RegisterType((*MsgCreateGauge)(nil), "osmosis.incentives.MsgCreateGauge")
	proto.RegisterType((*MsgCreateGaugeResponse)(nil), "osmosis.incentives.MsgCreateGaugeResponse")
	proto.RegisterType((*MsgAddToGauge)(nil), "osmosis.incentives.MsgAddToGauge")
	proto.RegisterType((*MsgAddToGaugeResponse)(nil), "osmosis.incentives.MsgAddToGaugeResponse")
}

func init() { proto.RegisterFile("osmosis/v9/incentives/tx.proto", fileDescriptor_c759ef67f8072133) }

var fileDescriptor_c759ef67f8072133 = []byte{
	// 598 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xac, 0x54, 0xcf, 0x6e, 0xd3, 0x30,
	0x18, 0xaf, 0x69, 0xf7, 0xcf, 0xdd, 0xd0, 0x88, 0x06, 0x64, 0xd5, 0x94, 0x74, 0x39, 0xa0, 0x80,
	0x84, 0xcd, 0x86, 0x84, 0x04, 0x37, 0x3a, 0x21, 0xb4, 0xc3, 0xc4, 0x88, 0x26, 0x21, 0x4d, 0x42,
	0x91, 0x93, 0x98, 0xcc, 0x5a, 0x13, 0x07, 0xdb, 0xe9, 0xd8, 0x5b, 0x4c, 0xe2, 0x2d, 0x78, 0x03,
	0x6e, 0x1c, 0x77, 0xdc, 0x91, 0x53, 0x87, 0xda, 0x37, 0xd8, 0x13, 0xa0, 0x38, 0xc9, 0xda, 0x0a,
	0xc6, 0x2e, 0x9c, 0x1c, 0xfb, 0xf7, 0xfb, 0x3e, 0x7f, 0xdf, 0xef, 0xf7, 0xc5, 0xd0, 0xe2, 0x32,
	0xe1, 0x92, 0x49, 0x3c, 0x78, 0x89, 0x59, 0x1a, 0xd2, 0x54, 0xb1, 0x01, 0x95, 0x58, 0x7d, 0x41,
	0x99, 0xe0, 0x8a, 0x1b, 0x46, 0x85, 0xa3, 0x09, 0xd8, 0x59, 0x8b, 0x79, 0xcc, 0x35, 0x8c, 0x8b,
	0xaf, 0x92, 0xd9, 0xb1, 0x63, 0xce, 0xe3, 0x3e, 0xc5, 0x7a, 0x17, 0xe4, 0x9f, 0xb0, 0x62, 0x09,
	0x95, 0x8a, 0x24, 0x59, 0x45, 0xb0, 0x42, 0x9d, 0x0b, 0x07, 0x44, 0x52, 0x3c, 0xd8, 0x0a, 0xa8,
	0x22, 0x5b, 0x38, 0xe4, 0x2c, 0xad, 0xf0, 0xcd, 0xbf, 0x97, 0x12, 0x93, 0x3c, 0xa6, 0x15, 0x65,
	0x63, 0x8a, 0xd2, 0xe7, 0xe1, 0x71, 0x9e, 0xe9, 0xa5, 0x44, 0x9d, 0xaf, 0x4d, 0x78, 0x77, 0x4f,
	0xc6, 0x3b, 0x82, 0x12, 0x45, 0xdf, 0x16, 0x61, 0xc6, 0x26, 0x5c, 0x66, 0xd2, 0xcf, 0xa8, 0xc8,
	0xa8, 0xca, 0x49, 0xdf, 0x04, 0x5d, 0xe0, 0x2e, 0x7a, 0x6d, 0x26, 0xf7, 0xeb, 0x23, 0xe3, 0x11,
	0x9c, 0xe3, 0x27, 0x29, 0x15, 0xe6, 0x9d, 0x2e, 0x70, 0x97, 0x7a, 0xab, 0x57, 0x43, 0x7b, 0xf9,
	0x94, 0x24, 0xfd, 0x57, 0x8e, 0x3e, 0x76, 0xbc, 0x12, 0x36, 0x76, 0xe1, 0x4a, 0xc4, 0xa4, 0x12,
	0x2c, 0xc8, 0x15, 0xf5, 0x15, 0x37, 0x9b, 0x5d, 0xe0, 0xb6, 0xb7, 0x2d, 0x54, 0x2b, 0x54, 0x16,
	0x84, 0xde, 0xe7, 0x54, 0x9c, 0xee, 0xf0, 0x34, 0x62, 0x8a, 0xf1, 0xb4, 0xd7, 0x3a, 0x1f, 0xda,
	0x0d, 0x6f, 0x79, 0x12, 0x7a, 0xc0, 0x0d, 0x02, 0xe7, 0x8a, 0xbe, 0xa5, 0xd9, 0xea, 0x36, 0xdd,
	0xf6, 0xf6, 0x3a, 0x2a, 0x95, 0x41, 0x85, 0x32, 0xa8, 0x52, 0x06, 0xed, 0x70, 0x96, 0xf6, 0x9e,
	0x15, 0xd1, 0xdf, 0x2e, 0x6d, 0x37, 0x66, 0xea, 0x28, 0x0f, 0x50, 0xc8, 0x13, 0x5c, 0xc9, 0x58,
	0x2e, 0x4f, 0x65, 0x74, 0x8c, 0xd5, 0x69, 0x46, 0xa5, 0x0e, 0x90, 0x5e, 0x99, 0xd9, 0xf8, 0x00,
	0xa1, 0x54, 0x44, 0x28, 0xbf, 0x70, 0xc1, 0x9c, 0xd3, 0xa5, 0x76, 0x50, 0x69, 0x11, 0xaa, 0x2d,
	0x42, 0x07, 0xb5, 0x45, 0xbd, 0x8d, 0xe2, 0xa2, 0xab, 0xa1, 0xbd, 0x5a, 0xb6, 0x7e, 0xed, 0x9d,
	0x73, 0x76, 0x69, 0x03, 0x6f, 0x49, 0xe7, 0x2a, 0xd8, 0x06, 0x86, 0x6b, 0x69, 0x9e, 0xf8, 0x34,
	0xe3, 0xe1, 0x91, 0xf4, 0x33, 0xc2, 0x22, 0x9f, 0x0f, 0xa8, 0x30, 0xe7, 0xbb, 0xc0, 0x6d, 0x79,
	0xf7, 0xd2, 0x3c, 0x79, 0xa3, 0xa1, 0x7d, 0xc2, 0xa2, 0x77, 0x03, 0x2a, 0x1c, 0x13, 0x3e, 0x98,
	0x35, 0xc5, 0xa3, 0x32, 0xe3, 0xa9, 0xa4, 0xce, 0x77, 0x00, 0x57, 0xf6, 0x64, 0xfc, 0x3a, 0x8a,
	0x0e, 0x78, 0x69, 0xd7, 0xb5, 0x17, 0xe0, 0xdf, 0x5e, 0xac, 0xc3, 0x45, 0x3d, 0x16, 0x3e, 0x8b,
	0xb4, 0x6d, 0x2d, 0x6f, 0x41, 0xef, 0x77, 0x23, 0x83, 0xc2, 0x05, 0x41, 0x4f, 0x88, 0x88, 0xa4,
	0xd9, 0xfc, 0xff, 0xea, 0xd6, 0xb9, 0x9d, 0x87, 0xf0, 0xfe, 0x4c, 0xe9, 0x75, 0x53, 0xdb, 0x3f,
	0x00, 0x6c, 0xee, 0xc9, 0xd8, 0xf8, 0x08, 0xdb, 0xd3, 0x83, 0xe8, 0xa0, 0x3f, 0x7f, 0x24, 0x34,
	0xab, 0x4b, 0xe7, 0xc9, 0xed, 0x9c, 0xfa, 0x1a, 0xe3, 0x10, 0xc2, 0x29, 0xdd, 0x36, 0x6f, 0x88,
	0x9c, 0x50, 0x3a, 0x8f, 0x6f, 0xa5, 0xd4, 0xb9, 0x7b, 0xfb, 0xe7, 0x23, 0x0b, 0x5c, 0x8c, 0x2c,
	0xf0, 0x6b, 0x64, 0x81, 0xb3, 0xb1, 0xd5, 0xb8, 0x18, 0x5b, 0x8d, 0x9f, 0x63, 0xab, 0x71, 0xf8,
	0x62, 0x4a, 0x28, 0x12, 0x90, 0x18, 0x7f, 0xce, 0x89, 0x24, 0x22, 0xe5, 0x11, 0xc5, 0x37, 0x3c,
	0x24, 0x85, 0x78, 0xc1, 0xbc, 0x9e, 0xb8, 0xe7, 0xbf, 0x03, 0x00, 0x00, 0xff, 0xff, 0x18, 0xbf,
	0x78, 0x48, 0x6e, 0x04, 0x00, 0x00,
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
	CreateGauge(ctx context.Context, in *MsgCreateGauge, opts ...grpc.CallOption) (*MsgCreateGaugeResponse, error)
	AddToGauge(ctx context.Context, in *MsgAddToGauge, opts ...grpc.CallOption) (*MsgAddToGaugeResponse, error)
}

type msgClient struct {
	cc grpc1.ClientConn
}

func NewMsgClient(cc grpc1.ClientConn) MsgClient {
	return &msgClient{cc}
}

func (c *msgClient) CreateGauge(ctx context.Context, in *MsgCreateGauge, opts ...grpc.CallOption) (*MsgCreateGaugeResponse, error) {
	out := new(MsgCreateGaugeResponse)
	err := c.cc.Invoke(ctx, "/osmosis.incentives.Msg/CreateGauge", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *msgClient) AddToGauge(ctx context.Context, in *MsgAddToGauge, opts ...grpc.CallOption) (*MsgAddToGaugeResponse, error) {
	out := new(MsgAddToGaugeResponse)
	err := c.cc.Invoke(ctx, "/osmosis.incentives.Msg/AddToGauge", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MsgServer is the server API for Msg service.
type MsgServer interface {
	CreateGauge(context.Context, *MsgCreateGauge) (*MsgCreateGaugeResponse, error)
	AddToGauge(context.Context, *MsgAddToGauge) (*MsgAddToGaugeResponse, error)
}

// UnimplementedMsgServer can be embedded to have forward compatible implementations.
type UnimplementedMsgServer struct {
}

func (*UnimplementedMsgServer) CreateGauge(ctx context.Context, req *MsgCreateGauge) (*MsgCreateGaugeResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateGauge not implemented")
}
func (*UnimplementedMsgServer) AddToGauge(ctx context.Context, req *MsgAddToGauge) (*MsgAddToGaugeResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddToGauge not implemented")
}

func RegisterMsgServer(s grpc1.Server, srv MsgServer) {
	s.RegisterService(&_Msg_serviceDesc, srv)
}

func _Msg_CreateGauge_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(MsgCreateGauge)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MsgServer).CreateGauge(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/osmosis.incentives.Msg/CreateGauge",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MsgServer).CreateGauge(ctx, req.(*MsgCreateGauge))
	}
	return interceptor(ctx, in, info, handler)
}

func _Msg_AddToGauge_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(MsgAddToGauge)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MsgServer).AddToGauge(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/osmosis.incentives.Msg/AddToGauge",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MsgServer).AddToGauge(ctx, req.(*MsgAddToGauge))
	}
	return interceptor(ctx, in, info, handler)
}

var _Msg_serviceDesc = grpc.ServiceDesc{
	ServiceName: "osmosis.incentives.Msg",
	HandlerType: (*MsgServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "CreateGauge",
			Handler:    _Msg_CreateGauge_Handler,
		},
		{
			MethodName: "AddToGauge",
			Handler:    _Msg_AddToGauge_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "osmosis/v9/incentives/tx.proto",
}

func (m *MsgCreateGauge) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgCreateGauge) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgCreateGauge) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.NumEpochsPaidOver != 0 {
		i = encodeVarintTx(dAtA, i, uint64(m.NumEpochsPaidOver))
		i--
		dAtA[i] = 0x30
	}
	n1, err1 := github_com_gogo_protobuf_types.StdTimeMarshalTo(m.StartTime, dAtA[i-github_com_gogo_protobuf_types.SizeOfStdTime(m.StartTime):])
	if err1 != nil {
		return 0, err1
	}
	i -= n1
	i = encodeVarintTx(dAtA, i, uint64(n1))
	i--
	dAtA[i] = 0x2a
	if len(m.Coins) > 0 {
		for iNdEx := len(m.Coins) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.Coins[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintTx(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x22
		}
	}
	{
		size, err := m.DistributeTo.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintTx(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x1a
	if len(m.Owner) > 0 {
		i -= len(m.Owner)
		copy(dAtA[i:], m.Owner)
		i = encodeVarintTx(dAtA, i, uint64(len(m.Owner)))
		i--
		dAtA[i] = 0x12
	}
	if m.IsPerpetual {
		i--
		if m.IsPerpetual {
			dAtA[i] = 1
		} else {
			dAtA[i] = 0
		}
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *MsgCreateGaugeResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgCreateGaugeResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgCreateGaugeResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	return len(dAtA) - i, nil
}

func (m *MsgAddToGauge) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgAddToGauge) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgAddToGauge) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Rewards) > 0 {
		for iNdEx := len(m.Rewards) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.Rewards[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintTx(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x1a
		}
	}
	if m.GaugeId != 0 {
		i = encodeVarintTx(dAtA, i, uint64(m.GaugeId))
		i--
		dAtA[i] = 0x10
	}
	if len(m.Owner) > 0 {
		i -= len(m.Owner)
		copy(dAtA[i:], m.Owner)
		i = encodeVarintTx(dAtA, i, uint64(len(m.Owner)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *MsgAddToGaugeResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MsgAddToGaugeResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *MsgAddToGaugeResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
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
func (m *MsgCreateGauge) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.IsPerpetual {
		n += 2
	}
	l = len(m.Owner)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	l = m.DistributeTo.Size()
	n += 1 + l + sovTx(uint64(l))
	if len(m.Coins) > 0 {
		for _, e := range m.Coins {
			l = e.Size()
			n += 1 + l + sovTx(uint64(l))
		}
	}
	l = github_com_gogo_protobuf_types.SizeOfStdTime(m.StartTime)
	n += 1 + l + sovTx(uint64(l))
	if m.NumEpochsPaidOver != 0 {
		n += 1 + sovTx(uint64(m.NumEpochsPaidOver))
	}
	return n
}

func (m *MsgCreateGaugeResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	return n
}

func (m *MsgAddToGauge) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.Owner)
	if l > 0 {
		n += 1 + l + sovTx(uint64(l))
	}
	if m.GaugeId != 0 {
		n += 1 + sovTx(uint64(m.GaugeId))
	}
	if len(m.Rewards) > 0 {
		for _, e := range m.Rewards {
			l = e.Size()
			n += 1 + l + sovTx(uint64(l))
		}
	}
	return n
}

func (m *MsgAddToGaugeResponse) Size() (n int) {
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
func (m *MsgCreateGauge) Unmarshal(dAtA []byte) error {
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
			return fmt.Errorf("proto: MsgCreateGauge: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgCreateGauge: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field IsPerpetual", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.IsPerpetual = bool(v != 0)
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Owner", wireType)
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
			m.Owner = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field DistributeTo", wireType)
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
			if err := m.DistributeTo.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Coins", wireType)
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
			m.Coins = append(m.Coins, types1.Coin{})
			if err := m.Coins[len(m.Coins)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 5:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartTime", wireType)
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
			if err := github_com_gogo_protobuf_types.StdTimeUnmarshal(&m.StartTime, dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 6:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field NumEpochsPaidOver", wireType)
			}
			m.NumEpochsPaidOver = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.NumEpochsPaidOver |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
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
func (m *MsgCreateGaugeResponse) Unmarshal(dAtA []byte) error {
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
			return fmt.Errorf("proto: MsgCreateGaugeResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgCreateGaugeResponse: illegal tag %d (wire type %d)", fieldNum, wire)
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
func (m *MsgAddToGauge) Unmarshal(dAtA []byte) error {
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
			return fmt.Errorf("proto: MsgAddToGauge: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgAddToGauge: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Owner", wireType)
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
			m.Owner = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field GaugeId", wireType)
			}
			m.GaugeId = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTx
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.GaugeId |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Rewards", wireType)
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
			m.Rewards = append(m.Rewards, types1.Coin{})
			if err := m.Rewards[len(m.Rewards)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
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
func (m *MsgAddToGaugeResponse) Unmarshal(dAtA []byte) error {
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
			return fmt.Errorf("proto: MsgAddToGaugeResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MsgAddToGaugeResponse: illegal tag %d (wire type %d)", fieldNum, wire)
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
