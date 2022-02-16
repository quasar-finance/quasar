// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: osmosis/gamm/pool-models/balancer/balancerPool.proto

// package osmosis.gamm.poolmodels;

package balancer

import (
	fmt "fmt"
	types "github.com/abag/quasarnode/x/gamm/types"
	github_com_cosmos_cosmos_sdk_types "github.com/cosmos/cosmos-sdk/types"
	types1 "github.com/cosmos/cosmos-sdk/types"
	_ "github.com/cosmos/cosmos-sdk/x/auth/types"
	_ "github.com/gogo/protobuf/gogoproto"
	proto "github.com/gogo/protobuf/proto"
	github_com_gogo_protobuf_types "github.com/gogo/protobuf/types"
	_ "github.com/regen-network/cosmos-proto"
	_ "google.golang.org/protobuf/types/known/durationpb"
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

// Parameters for changing the weights in a balancer pool smoothly from
// a start weight and end weight over a period of time.
// Currently, the only smooth change supported is linear changing between
// the two weights, but more types may be added in the future.
// When these parameters are set, the weight w(t) for pool time `t` is the
// following:
//   t <= start_time: w(t) = initial_pool_weights
//   start_time < t <= start_time + duration:
//     w(t) = initial_pool_weights + (t - start_time) *
//       (target_pool_weights - initial_pool_weights) / (duration)
//   t > start_time + duration: w(t) = target_pool_weights
type SmoothWeightChangeParams struct {
	// The start time for beginning the weight change.
	// If a parameter change / pool instantiation leaves this blank,
	// it should be generated by the state_machine as the current time.
	StartTime time.Time `protobuf:"bytes,1,opt,name=start_time,json=startTime,proto3,stdtime" json:"start_time" yaml:"start_time"`
	// Duration for the weights to change over
	Duration time.Duration `protobuf:"bytes,2,opt,name=duration,proto3,stdduration" json:"duration,omitempty" yaml:"duration"`
	// The initial pool weights. These are copied from the pool's settings
	// at the time of weight change instantiation.
	// The amount PoolAsset.token.amount field is ignored if present,
	// future type refactorings should just have a type with the denom & weight
	// here.
	InitialPoolWeights []types.PoolAsset `protobuf:"bytes,3,rep,name=initialPoolWeights,proto3" json:"initialPoolWeights" yaml:"initial_pool_weights"`
	// The target pool weights. The pool weights will change linearly with respect
	// to time between start_time, and start_time + duration. The amount
	// PoolAsset.token.amount field is ignored if present, future type
	// refactorings should just have a type with the denom & weight here.
	TargetPoolWeights []types.PoolAsset `protobuf:"bytes,4,rep,name=targetPoolWeights,proto3" json:"targetPoolWeights" yaml:"target_pool_weights"`
}

func (m *SmoothWeightChangeParams) Reset()         { *m = SmoothWeightChangeParams{} }
func (m *SmoothWeightChangeParams) String() string { return proto.CompactTextString(m) }
func (*SmoothWeightChangeParams) ProtoMessage()    {}
func (*SmoothWeightChangeParams) Descriptor() ([]byte, []int) {
	return fileDescriptor_7e991f749f68c2a4, []int{0}
}
func (m *SmoothWeightChangeParams) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *SmoothWeightChangeParams) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_SmoothWeightChangeParams.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *SmoothWeightChangeParams) XXX_Merge(src proto.Message) {
	xxx_messageInfo_SmoothWeightChangeParams.Merge(m, src)
}
func (m *SmoothWeightChangeParams) XXX_Size() int {
	return m.Size()
}
func (m *SmoothWeightChangeParams) XXX_DiscardUnknown() {
	xxx_messageInfo_SmoothWeightChangeParams.DiscardUnknown(m)
}

var xxx_messageInfo_SmoothWeightChangeParams proto.InternalMessageInfo

func (m *SmoothWeightChangeParams) GetStartTime() time.Time {
	if m != nil {
		return m.StartTime
	}
	return time.Time{}
}

func (m *SmoothWeightChangeParams) GetDuration() time.Duration {
	if m != nil {
		return m.Duration
	}
	return 0
}

func (m *SmoothWeightChangeParams) GetInitialPoolWeights() []types.PoolAsset {
	if m != nil {
		return m.InitialPoolWeights
	}
	return nil
}

func (m *SmoothWeightChangeParams) GetTargetPoolWeights() []types.PoolAsset {
	if m != nil {
		return m.TargetPoolWeights
	}
	return nil
}

// BalancerPoolParams defined the parameters that will be managed by the pool
// governance in the future. This params are not managed by the chain
// governance. Instead they will be managed by the token holders of the pool.
// The pool's token holders are specified in future_pool_governor.
type BalancerPoolParams struct {
	SwapFee                  github_com_cosmos_cosmos_sdk_types.Dec `protobuf:"bytes,1,opt,name=swapFee,proto3,customtype=github.com/cosmos/cosmos-sdk/types.Dec" json:"swapFee" yaml:"swap_fee"`
	ExitFee                  github_com_cosmos_cosmos_sdk_types.Dec `protobuf:"bytes,2,opt,name=exitFee,proto3,customtype=github.com/cosmos/cosmos-sdk/types.Dec" json:"exitFee" yaml:"exit_fee"`
	SmoothWeightChangeParams *SmoothWeightChangeParams              `protobuf:"bytes,3,opt,name=smoothWeightChangeParams,proto3" json:"smoothWeightChangeParams,omitempty" yaml:"smooth_weight_change_params"`
}

func (m *BalancerPoolParams) Reset()         { *m = BalancerPoolParams{} }
func (m *BalancerPoolParams) String() string { return proto.CompactTextString(m) }
func (*BalancerPoolParams) ProtoMessage()    {}
func (*BalancerPoolParams) Descriptor() ([]byte, []int) {
	return fileDescriptor_7e991f749f68c2a4, []int{1}
}
func (m *BalancerPoolParams) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *BalancerPoolParams) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_BalancerPoolParams.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *BalancerPoolParams) XXX_Merge(src proto.Message) {
	xxx_messageInfo_BalancerPoolParams.Merge(m, src)
}
func (m *BalancerPoolParams) XXX_Size() int {
	return m.Size()
}
func (m *BalancerPoolParams) XXX_DiscardUnknown() {
	xxx_messageInfo_BalancerPoolParams.DiscardUnknown(m)
}

var xxx_messageInfo_BalancerPoolParams proto.InternalMessageInfo

func (m *BalancerPoolParams) GetSmoothWeightChangeParams() *SmoothWeightChangeParams {
	if m != nil {
		return m.SmoothWeightChangeParams
	}
	return nil
}

type BalancerPool struct {
	Address    string             `protobuf:"bytes,1,opt,name=address,proto3" json:"address,omitempty" yaml:"address"`
	Id         uint64             `protobuf:"varint,2,opt,name=id,proto3" json:"id,omitempty"`
	PoolParams BalancerPoolParams `protobuf:"bytes,3,opt,name=poolParams,proto3" json:"poolParams" yaml:"balancer_pool_params"`
	// This string specifies who will govern the pool in the future.
	// Valid forms of this are:
	// {token name},{duration}
	// {duration}
	// where {token name} if specified is the token which determines the
	// governor, and if not specified is the LP token for this pool.duration is
	// a time specified as 0w,1w,2w, etc. which specifies how long the token
	// would need to be locked up to count in governance. 0w means no lockup.
	// TODO: Further improve these docs
	FuturePoolGovernor string `protobuf:"bytes,4,opt,name=future_pool_governor,json=futurePoolGovernor,proto3" json:"future_pool_governor,omitempty" yaml:"future_pool_governor"`
	// sum of all LP tokens sent out
	TotalShares types1.Coin `protobuf:"bytes,5,opt,name=totalShares,proto3" json:"totalShares" yaml:"total_shares"`
	// These are assumed to be sorted by denomiation.
	// They contain the pool asset and the information about the weight
	PoolAssets []types.PoolAsset `protobuf:"bytes,6,rep,name=poolAssets,proto3" json:"poolAssets" yaml:"pool_assets"`
	// sum of all non-normalized pool weights
	TotalWeight github_com_cosmos_cosmos_sdk_types.Int `protobuf:"bytes,7,opt,name=totalWeight,proto3,customtype=github.com/cosmos/cosmos-sdk/types.Int" json:"totalWeight" yaml:"total_weight"`
}

func (m *BalancerPool) Reset()         { *m = BalancerPool{} }
func (m *BalancerPool) String() string { return proto.CompactTextString(m) }
func (*BalancerPool) ProtoMessage()    {}
func (*BalancerPool) Descriptor() ([]byte, []int) {
	return fileDescriptor_7e991f749f68c2a4, []int{2}
}
func (m *BalancerPool) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *BalancerPool) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_BalancerPool.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *BalancerPool) XXX_Merge(src proto.Message) {
	xxx_messageInfo_BalancerPool.Merge(m, src)
}
func (m *BalancerPool) XXX_Size() int {
	return m.Size()
}
func (m *BalancerPool) XXX_DiscardUnknown() {
	xxx_messageInfo_BalancerPool.DiscardUnknown(m)
}

var xxx_messageInfo_BalancerPool proto.InternalMessageInfo

func (m *BalancerPool) GetAddress() string {
	if m != nil {
		return m.Address
	}
	return ""
}

func (m *BalancerPool) GetId() uint64 {
	if m != nil {
		return m.Id
	}
	return 0
}

func (m *BalancerPool) GetPoolParams() BalancerPoolParams {
	if m != nil {
		return m.PoolParams
	}
	return BalancerPoolParams{}
}

func (m *BalancerPool) GetFuturePoolGovernor() string {
	if m != nil {
		return m.FuturePoolGovernor
	}
	return ""
}

func (m *BalancerPool) GetTotalShares() types1.Coin {
	if m != nil {
		return m.TotalShares
	}
	return types1.Coin{}
}

func (m *BalancerPool) GetPoolAssets() []types.PoolAsset {
	if m != nil {
		return m.PoolAssets
	}
	return nil
}

func init() {
	proto.RegisterType((*SmoothWeightChangeParams)(nil), "abag.quasarnode.osmosis.gamm.poolmodels.SmoothWeightChangeParams")
	proto.RegisterType((*BalancerPoolParams)(nil), "abag.quasarnode.osmosis.gamm.poolmodels.BalancerPoolParams")
	proto.RegisterType((*BalancerPool)(nil), "abag.quasarnode.osmosis.gamm.poolmodels.BalancerPool")
}

func init() {
	proto.RegisterFile("osmosis/gamm/pool-models/balancer/balancerPool.proto", fileDescriptor_7e991f749f68c2a4)
}

var fileDescriptor_7e991f749f68c2a4 = []byte{
	// 774 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xa4, 0x95, 0xcf, 0x4e, 0xdb, 0x4a,
	0x14, 0xc6, 0xe3, 0x24, 0x17, 0x2e, 0xc3, 0x15, 0x57, 0xcc, 0x65, 0x61, 0x82, 0x6e, 0x8c, 0x5c,
	0xa9, 0xa0, 0x0a, 0xc6, 0x82, 0x56, 0x5d, 0xb4, 0x2b, 0x0c, 0x6d, 0xd5, 0x1d, 0x35, 0x95, 0x8a,
	0xda, 0x85, 0x35, 0x89, 0x07, 0xc7, 0xaa, 0xed, 0x71, 0x3d, 0x13, 0x20, 0x52, 0x17, 0x7d, 0x04,
	0x96, 0x7d, 0x85, 0xbe, 0x09, 0xea, 0x8a, 0x5d, 0xab, 0x2e, 0xdc, 0x0a, 0x76, 0x5d, 0x66, 0xd9,
	0x55, 0x35, 0x7f, 0x9c, 0xa4, 0x81, 0x54, 0xa0, 0xae, 0x88, 0xe7, 0x9c, 0xf9, 0x7d, 0xe7, 0x3b,
	0x73, 0x66, 0x00, 0xf7, 0x28, 0x4b, 0x28, 0x8b, 0x98, 0x13, 0xe2, 0x24, 0x71, 0x32, 0x4a, 0xe3,
	0xf5, 0x84, 0x06, 0x24, 0x66, 0x4e, 0x0b, 0xc7, 0x38, 0x6d, 0x93, 0x7c, 0xf0, 0x63, 0x97, 0xd2,
	0x18, 0x65, 0x39, 0xe5, 0x14, 0xae, 0xe0, 0x16, 0x0e, 0xd1, 0x9b, 0x2e, 0x66, 0x38, 0x4f, 0x69,
	0x40, 0x90, 0xa6, 0x20, 0x41, 0x41, 0x82, 0xa2, 0x20, 0x8d, 0xc5, 0xb6, 0x8c, 0xf8, 0x72, 0x9b,
	0xa3, 0x3e, 0x14, 0xa3, 0xb1, 0x10, 0xd2, 0x90, 0xaa, 0x75, 0xf1, 0x4b, 0xaf, 0x36, 0x43, 0x4a,
	0xc3, 0x98, 0x38, 0xf2, 0xab, 0xd5, 0x3d, 0x70, 0x82, 0x6e, 0x8e, 0x79, 0x44, 0x53, 0x1d, 0xb7,
	0xc6, 0xe3, 0x3c, 0x4a, 0x08, 0xe3, 0x38, 0xc9, 0x4a, 0x80, 0x12, 0x71, 0x70, 0x97, 0x77, 0x9c,
	0xc3, 0x8d, 0x16, 0xe1, 0x78, 0x43, 0x7e, 0x8c, 0xc5, 0x5b, 0x98, 0x91, 0x41, 0xbc, 0x4d, 0xa3,
	0x81, 0xc0, 0x2f, 0x0d, 0x29, 0x13, 0xb2, 0x81, 0x77, 0xfb, 0x53, 0x0d, 0x98, 0x7b, 0x09, 0xa5,
	0xbc, 0xf3, 0x82, 0x44, 0x61, 0x87, 0x6f, 0x77, 0x70, 0x1a, 0x92, 0x5d, 0x9c, 0xe3, 0x84, 0xc1,
	0x7d, 0x00, 0x18, 0xc7, 0x39, 0xf7, 0x45, 0x59, 0xa6, 0xb1, 0x6c, 0xac, 0xce, 0x6e, 0x36, 0x90,
	0xaa, 0x19, 0x95, 0x35, 0xa3, 0xe7, 0x65, 0xcd, 0xee, 0xff, 0xa7, 0x85, 0x55, 0xe9, 0x17, 0xd6,
	0x7c, 0x0f, 0x27, 0xf1, 0x03, 0x7b, 0xb8, 0xd7, 0x3e, 0xf9, 0x6a, 0x19, 0xde, 0x8c, 0x5c, 0x10,
	0xe9, 0xb0, 0x03, 0xfe, 0x2e, 0x5b, 0x61, 0x56, 0x25, 0x77, 0xf1, 0x12, 0x77, 0x47, 0x27, 0xb8,
	0x1b, 0x02, 0xfb, 0xbd, 0xb0, 0x60, 0xb9, 0x65, 0x8d, 0x26, 0x11, 0x27, 0x49, 0xc6, 0x7b, 0xfd,
	0xc2, 0xfa, 0x57, 0x89, 0x95, 0x31, 0xfb, 0xbd, 0x90, 0x1a, 0xd0, 0xe1, 0x5b, 0x00, 0xa3, 0x34,
	0xe2, 0x11, 0x8e, 0xc5, 0x89, 0x2b, 0x93, 0xcc, 0xac, 0x2d, 0xd7, 0x56, 0x67, 0x37, 0x57, 0xd0,
	0x6f, 0x4f, 0x5e, 0x6c, 0xd8, 0x62, 0x8c, 0x70, 0xf7, 0x96, 0x36, 0xb6, 0xa4, 0xb4, 0x34, 0xd0,
	0x17, 0x7d, 0xf4, 0x8f, 0x14, 0xd2, 0xf6, 0xae, 0xd0, 0x81, 0x3d, 0x30, 0xcf, 0x71, 0x1e, 0x12,
	0x3e, 0x2a, 0x5e, 0xbf, 0x99, 0xb8, 0xad, 0xc5, 0x1b, 0x4a, 0x5c, 0xf1, 0xc6, 0xb4, 0x2f, 0xab,
	0xd8, 0x3f, 0xaa, 0x00, 0xba, 0x23, 0xc3, 0xae, 0xcf, 0xf4, 0x15, 0x98, 0x66, 0x47, 0x38, 0x7b,
	0x4c, 0xd4, 0x81, 0xce, 0xb8, 0x5b, 0x02, 0xff, 0xa5, 0xb0, 0x6e, 0x87, 0x11, 0xef, 0x74, 0x5b,
	0xa8, 0x4d, 0x13, 0x3d, 0xda, 0xfa, 0xcf, 0x3a, 0x0b, 0x5e, 0x3b, 0xbc, 0x97, 0x11, 0x86, 0x76,
	0x48, 0x7b, 0xd8, 0x71, 0x81, 0xf1, 0x0f, 0x08, 0xb1, 0xbd, 0x92, 0x28, 0xe0, 0xe4, 0x38, 0xe2,
	0x02, 0x5e, 0xfd, 0x33, 0xb8, 0xc0, 0x68, 0xb8, 0x26, 0xc2, 0x0f, 0x06, 0x30, 0xd9, 0x84, 0x51,
	0x35, 0x6b, 0x72, 0x88, 0xb6, 0xd0, 0x35, 0xaf, 0x32, 0x9a, 0x34, 0xf3, 0xee, 0x9d, 0xd3, 0xc2,
	0x32, 0xfa, 0x85, 0x65, 0x6b, 0x93, 0x32, 0x4f, 0x37, 0xda, 0x6f, 0xcb, 0x4c, 0x3f, 0x93, 0xa9,
	0xb6, 0x37, 0xb1, 0x1c, 0xfb, 0x63, 0x1d, 0xfc, 0x33, 0xda, 0x7c, 0xb8, 0x06, 0xa6, 0x71, 0x10,
	0xe4, 0x84, 0x31, 0xdd, 0x76, 0xd8, 0x2f, 0xac, 0x39, 0xa5, 0xa1, 0x03, 0xb6, 0x57, 0xa6, 0xc0,
	0x39, 0x50, 0x8d, 0x02, 0xd9, 0xc2, 0xba, 0x57, 0x8d, 0x02, 0xf8, 0xce, 0x00, 0x20, 0x1b, 0x9c,
	0xa1, 0x36, 0xfb, 0xf0, 0xda, 0x66, 0x2f, 0x8f, 0xc1, 0xf8, 0x44, 0x97, 0xaf, 0xa2, 0x1a, 0xab,
	0xd2, 0xdf, 0x88, 0x26, 0x7c, 0x06, 0x16, 0x0e, 0xba, 0xbc, 0x9b, 0x13, 0x95, 0x12, 0xd2, 0x43,
	0x92, 0xa7, 0x34, 0x37, 0xeb, 0xd2, 0x8d, 0x35, 0x44, 0x5d, 0x95, 0x65, 0x7b, 0x50, 0x2d, 0x8b,
	0x0a, 0x9e, 0xe8, 0x45, 0xb8, 0x0f, 0x66, 0x39, 0xe5, 0x38, 0xde, 0xeb, 0xe0, 0x9c, 0x30, 0xf3,
	0x2f, 0xfd, 0x0e, 0xe8, 0x77, 0x55, 0x3c, 0x69, 0x48, 0xbf, 0x58, 0x68, 0x9b, 0x46, 0xa9, 0xbb,
	0xa4, 0x6b, 0xfe, 0x4f, 0x5f, 0x04, 0xb1, 0xd7, 0x67, 0x72, 0xb3, 0xed, 0x8d, 0xa2, 0x60, 0x5b,
	0xb5, 0x4b, 0xde, 0x1f, 0x66, 0x4e, 0xdd, 0xec, 0xbe, 0x35, 0xb4, 0x0c, 0x54, 0x32, 0xd2, 0x08,
	0x96, 0x24, 0xdd, 0x11, 0x85, 0x85, 0xa1, 0x2e, 0x5f, 0x1d, 0xbf, 0x39, 0x2d, 0x1b, 0xf1, 0xe8,
	0x06, 0x03, 0xff, 0x34, 0xe5, 0xe3, 0x6e, 0xd4, 0x9c, 0x95, 0x6e, 0x14, 0xd9, 0xdd, 0x3d, 0x3d,
	0x6f, 0x1a, 0x67, 0xe7, 0x4d, 0xe3, 0xdb, 0x79, 0xd3, 0x38, 0xb9, 0x68, 0x56, 0xce, 0x2e, 0x9a,
	0x95, 0xcf, 0x17, 0xcd, 0xca, 0xcb, 0xfb, 0x23, 0x2a, 0xc2, 0x9d, 0x33, 0x74, 0xe7, 0x1c, 0x4f,
	0xfe, 0x27, 0xd8, 0x9a, 0x92, 0x8f, 0xec, 0xdd, 0x9f, 0x01, 0x00, 0x00, 0xff, 0xff, 0xfd, 0x98,
	0x94, 0xdb, 0x30, 0x07, 0x00, 0x00,
}

func (m *SmoothWeightChangeParams) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *SmoothWeightChangeParams) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *SmoothWeightChangeParams) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.TargetPoolWeights) > 0 {
		for iNdEx := len(m.TargetPoolWeights) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.TargetPoolWeights[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintBalancerPool(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x22
		}
	}
	if len(m.InitialPoolWeights) > 0 {
		for iNdEx := len(m.InitialPoolWeights) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.InitialPoolWeights[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintBalancerPool(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x1a
		}
	}
	n1, err1 := github_com_gogo_protobuf_types.StdDurationMarshalTo(m.Duration, dAtA[i-github_com_gogo_protobuf_types.SizeOfStdDuration(m.Duration):])
	if err1 != nil {
		return 0, err1
	}
	i -= n1
	i = encodeVarintBalancerPool(dAtA, i, uint64(n1))
	i--
	dAtA[i] = 0x12
	n2, err2 := github_com_gogo_protobuf_types.StdTimeMarshalTo(m.StartTime, dAtA[i-github_com_gogo_protobuf_types.SizeOfStdTime(m.StartTime):])
	if err2 != nil {
		return 0, err2
	}
	i -= n2
	i = encodeVarintBalancerPool(dAtA, i, uint64(n2))
	i--
	dAtA[i] = 0xa
	return len(dAtA) - i, nil
}

func (m *BalancerPoolParams) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *BalancerPoolParams) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *BalancerPoolParams) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.SmoothWeightChangeParams != nil {
		{
			size, err := m.SmoothWeightChangeParams.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintBalancerPool(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x1a
	}
	{
		size := m.ExitFee.Size()
		i -= size
		if _, err := m.ExitFee.MarshalTo(dAtA[i:]); err != nil {
			return 0, err
		}
		i = encodeVarintBalancerPool(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x12
	{
		size := m.SwapFee.Size()
		i -= size
		if _, err := m.SwapFee.MarshalTo(dAtA[i:]); err != nil {
			return 0, err
		}
		i = encodeVarintBalancerPool(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0xa
	return len(dAtA) - i, nil
}

func (m *BalancerPool) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *BalancerPool) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *BalancerPool) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	{
		size := m.TotalWeight.Size()
		i -= size
		if _, err := m.TotalWeight.MarshalTo(dAtA[i:]); err != nil {
			return 0, err
		}
		i = encodeVarintBalancerPool(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x3a
	if len(m.PoolAssets) > 0 {
		for iNdEx := len(m.PoolAssets) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.PoolAssets[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintBalancerPool(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x32
		}
	}
	{
		size, err := m.TotalShares.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintBalancerPool(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x2a
	if len(m.FuturePoolGovernor) > 0 {
		i -= len(m.FuturePoolGovernor)
		copy(dAtA[i:], m.FuturePoolGovernor)
		i = encodeVarintBalancerPool(dAtA, i, uint64(len(m.FuturePoolGovernor)))
		i--
		dAtA[i] = 0x22
	}
	{
		size, err := m.PoolParams.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintBalancerPool(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x1a
	if m.Id != 0 {
		i = encodeVarintBalancerPool(dAtA, i, uint64(m.Id))
		i--
		dAtA[i] = 0x10
	}
	if len(m.Address) > 0 {
		i -= len(m.Address)
		copy(dAtA[i:], m.Address)
		i = encodeVarintBalancerPool(dAtA, i, uint64(len(m.Address)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func encodeVarintBalancerPool(dAtA []byte, offset int, v uint64) int {
	offset -= sovBalancerPool(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *SmoothWeightChangeParams) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = github_com_gogo_protobuf_types.SizeOfStdTime(m.StartTime)
	n += 1 + l + sovBalancerPool(uint64(l))
	l = github_com_gogo_protobuf_types.SizeOfStdDuration(m.Duration)
	n += 1 + l + sovBalancerPool(uint64(l))
	if len(m.InitialPoolWeights) > 0 {
		for _, e := range m.InitialPoolWeights {
			l = e.Size()
			n += 1 + l + sovBalancerPool(uint64(l))
		}
	}
	if len(m.TargetPoolWeights) > 0 {
		for _, e := range m.TargetPoolWeights {
			l = e.Size()
			n += 1 + l + sovBalancerPool(uint64(l))
		}
	}
	return n
}

func (m *BalancerPoolParams) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = m.SwapFee.Size()
	n += 1 + l + sovBalancerPool(uint64(l))
	l = m.ExitFee.Size()
	n += 1 + l + sovBalancerPool(uint64(l))
	if m.SmoothWeightChangeParams != nil {
		l = m.SmoothWeightChangeParams.Size()
		n += 1 + l + sovBalancerPool(uint64(l))
	}
	return n
}

func (m *BalancerPool) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.Address)
	if l > 0 {
		n += 1 + l + sovBalancerPool(uint64(l))
	}
	if m.Id != 0 {
		n += 1 + sovBalancerPool(uint64(m.Id))
	}
	l = m.PoolParams.Size()
	n += 1 + l + sovBalancerPool(uint64(l))
	l = len(m.FuturePoolGovernor)
	if l > 0 {
		n += 1 + l + sovBalancerPool(uint64(l))
	}
	l = m.TotalShares.Size()
	n += 1 + l + sovBalancerPool(uint64(l))
	if len(m.PoolAssets) > 0 {
		for _, e := range m.PoolAssets {
			l = e.Size()
			n += 1 + l + sovBalancerPool(uint64(l))
		}
	}
	l = m.TotalWeight.Size()
	n += 1 + l + sovBalancerPool(uint64(l))
	return n
}

func sovBalancerPool(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozBalancerPool(x uint64) (n int) {
	return sovBalancerPool(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *SmoothWeightChangeParams) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowBalancerPool
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
			return fmt.Errorf("proto: SmoothWeightChangeParams: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: SmoothWeightChangeParams: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartTime", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := github_com_gogo_protobuf_types.StdTimeUnmarshal(&m.StartTime, dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Duration", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := github_com_gogo_protobuf_types.StdDurationUnmarshal(&m.Duration, dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field InitialPoolWeights", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.InitialPoolWeights = append(m.InitialPoolWeights, types.PoolAsset{})
			if err := m.InitialPoolWeights[len(m.InitialPoolWeights)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field TargetPoolWeights", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.TargetPoolWeights = append(m.TargetPoolWeights, types.PoolAsset{})
			if err := m.TargetPoolWeights[len(m.TargetPoolWeights)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipBalancerPool(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthBalancerPool
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
func (m *BalancerPoolParams) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowBalancerPool
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
			return fmt.Errorf("proto: BalancerPoolParams: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: BalancerPoolParams: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field SwapFee", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.SwapFee.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ExitFee", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.ExitFee.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field SmoothWeightChangeParams", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.SmoothWeightChangeParams == nil {
				m.SmoothWeightChangeParams = &SmoothWeightChangeParams{}
			}
			if err := m.SmoothWeightChangeParams.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipBalancerPool(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthBalancerPool
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
func (m *BalancerPool) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowBalancerPool
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
			return fmt.Errorf("proto: BalancerPool: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: BalancerPool: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Address", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Address = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Id", wireType)
			}
			m.Id = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Id |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PoolParams", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.PoolParams.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field FuturePoolGovernor", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.FuturePoolGovernor = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 5:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field TotalShares", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.TotalShares.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 6:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PoolAssets", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.PoolAssets = append(m.PoolAssets, types.PoolAsset{})
			if err := m.PoolAssets[len(m.PoolAssets)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 7:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field TotalWeight", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowBalancerPool
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
				return ErrInvalidLengthBalancerPool
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthBalancerPool
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.TotalWeight.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipBalancerPool(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthBalancerPool
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
func skipBalancerPool(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowBalancerPool
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
					return 0, ErrIntOverflowBalancerPool
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
					return 0, ErrIntOverflowBalancerPool
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
				return 0, ErrInvalidLengthBalancerPool
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupBalancerPool
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthBalancerPool
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthBalancerPool        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowBalancerPool          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupBalancerPool = fmt.Errorf("proto: unexpected end of group")
)
