// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: quasarlabs/quasarnode/qoracle/osmosis/params.proto

package types

import (
	fmt "fmt"
	_ "github.com/cosmos/cosmos-sdk/types"
	_ "github.com/cosmos/gogoproto/gogoproto"
	proto "github.com/cosmos/gogoproto/proto"
	types "github.com/cosmos/ibc-go/v7/modules/core/02-client/types"
	_ "google.golang.org/protobuf/types/known/anypb"
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

type Params struct {
	Enabled                bool         `protobuf:"varint,1,opt,name=enabled,proto3" json:"enabled,omitempty" yaml:"enabled"`
	EpochIdentifier        string       `protobuf:"bytes,2,opt,name=epoch_identifier,json=epochIdentifier,proto3" json:"epoch_identifier,omitempty" yaml:"epoch_identifier"`
	AuthorizedChannel      string       `protobuf:"bytes,3,opt,name=authorized_channel,json=authorizedChannel,proto3" json:"authorized_channel,omitempty" yaml:"authorized_channel"`
	PacketTimeoutHeight    types.Height `protobuf:"bytes,4,opt,name=packet_timeout_height,json=packetTimeoutHeight,proto3" json:"packet_timeout_height" yaml:"packet_timeout_height"`
	PacketTimeoutTimestamp uint64       `protobuf:"varint,5,opt,name=packet_timeout_timestamp,json=packetTimeoutTimestamp,proto3" json:"packet_timeout_timestamp,omitempty" yaml:"packet_timeout_timestamp"`
}

func (m *Params) Reset()      { *m = Params{} }
func (*Params) ProtoMessage() {}
func (*Params) Descriptor() ([]byte, []int) {
	return fileDescriptor_c288508491767e26, []int{0}
}
func (m *Params) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *Params) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_Params.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *Params) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Params.Merge(m, src)
}
func (m *Params) XXX_Size() int {
	return m.Size()
}
func (m *Params) XXX_DiscardUnknown() {
	xxx_messageInfo_Params.DiscardUnknown(m)
}

var xxx_messageInfo_Params proto.InternalMessageInfo

func (m *Params) GetEnabled() bool {
	if m != nil {
		return m.Enabled
	}
	return false
}

func (m *Params) GetEpochIdentifier() string {
	if m != nil {
		return m.EpochIdentifier
	}
	return ""
}

func (m *Params) GetAuthorizedChannel() string {
	if m != nil {
		return m.AuthorizedChannel
	}
	return ""
}

func (m *Params) GetPacketTimeoutHeight() types.Height {
	if m != nil {
		return m.PacketTimeoutHeight
	}
	return types.Height{}
}

func (m *Params) GetPacketTimeoutTimestamp() uint64 {
	if m != nil {
		return m.PacketTimeoutTimestamp
	}
	return 0
}

func init() {
	proto.RegisterType((*Params)(nil), "quasarlabs.quasarnode.qoracle.osmosis.Params")
}

func init() {
	proto.RegisterFile("quasarlabs/quasarnode/qoracle/osmosis/params.proto", fileDescriptor_c288508491767e26)
}

var fileDescriptor_c288508491767e26 = []byte{
	// 448 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x6c, 0x92, 0xcd, 0x6e, 0xd3, 0x40,
	0x14, 0x85, 0x6d, 0x1a, 0x0a, 0x18, 0x89, 0x1f, 0xf3, 0xe7, 0x06, 0xf0, 0x44, 0x06, 0xa4, 0x2c,
	0xd0, 0x8c, 0x52, 0x36, 0xa8, 0x4b, 0x23, 0x21, 0x90, 0x58, 0x20, 0xab, 0x2b, 0x24, 0x14, 0xcd,
	0x4c, 0x6e, 0xed, 0x11, 0xb6, 0xc7, 0xf5, 0x8c, 0x23, 0xc2, 0x53, 0xb0, 0x64, 0xc9, 0x8e, 0x57,
	0xe9, 0xb2, 0x4b, 0x56, 0x16, 0x4a, 0xde, 0xc0, 0x4f, 0x80, 0xe2, 0x99, 0x36, 0x6a, 0x9b, 0x95,
	0xaf, 0xcf, 0x39, 0xdf, 0xb1, 0x75, 0x75, 0xbd, 0xfd, 0xe3, 0x86, 0x2a, 0x5a, 0xe7, 0x94, 0x29,
	0x62, 0xc6, 0x52, 0xce, 0x80, 0x1c, 0xcb, 0x9a, 0xf2, 0x1c, 0x88, 0x54, 0x85, 0x54, 0x42, 0x91,
	0x8a, 0xd6, 0xb4, 0x50, 0xb8, 0xaa, 0xa5, 0x96, 0xfe, 0xab, 0x0d, 0x83, 0x37, 0x0c, 0xb6, 0x0c,
	0xb6, 0xcc, 0xf0, 0x61, 0x2a, 0x53, 0xd9, 0x13, 0x64, 0x3d, 0x19, 0x78, 0xb8, 0x97, 0x4a, 0x99,
	0xe6, 0x40, 0xfa, 0x37, 0xd6, 0x1c, 0x11, 0x5a, 0x2e, 0xac, 0x15, 0xf2, 0x1e, 0x25, 0x8c, 0x2a,
	0x20, 0xf3, 0x09, 0x03, 0x4d, 0x27, 0x84, 0x4b, 0x51, 0x5a, 0x1f, 0x09, 0xc6, 0x09, 0x97, 0x35,
	0x10, 0x9e, 0x0b, 0x28, 0x35, 0x99, 0x4f, 0xec, 0x64, 0x02, 0xd1, 0x9f, 0x1d, 0x6f, 0xf7, 0x73,
	0xff, 0xa7, 0xfe, 0x6b, 0xef, 0x06, 0x94, 0x94, 0xe5, 0x30, 0x0b, 0xdc, 0x91, 0x3b, 0xbe, 0x19,
	0xfb, 0x5d, 0x8b, 0xee, 0x2c, 0x68, 0x91, 0x1f, 0x44, 0xd6, 0x88, 0x92, 0xb3, 0x88, 0xff, 0xde,
	0xbb, 0x07, 0x95, 0xe4, 0xd9, 0x54, 0xcc, 0xa0, 0xd4, 0xe2, 0x48, 0x40, 0x1d, 0x5c, 0x1b, 0xb9,
	0xe3, 0x5b, 0xf1, 0xd3, 0xae, 0x45, 0x4f, 0x2c, 0x76, 0x29, 0x11, 0x25, 0x77, 0x7b, 0xe9, 0xe3,
	0xb9, 0xe2, 0x7f, 0xf2, 0x7c, 0xda, 0xe8, 0x4c, 0xd6, 0xe2, 0x07, 0xcc, 0xa6, 0x3c, 0xa3, 0x65,
	0x09, 0x79, 0xb0, 0xd3, 0x37, 0x3d, 0xef, 0x5a, 0xb4, 0x67, 0x9a, 0xae, 0x66, 0xa2, 0xe4, 0xfe,
	0x46, 0x7c, 0x67, 0x34, 0x5f, 0x7b, 0x8f, 0x2a, 0xca, 0xbf, 0x81, 0x9e, 0x6a, 0x51, 0x80, 0x6c,
	0xf4, 0x34, 0x03, 0x91, 0x66, 0x3a, 0x18, 0x8c, 0xdc, 0xf1, 0xed, 0xfd, 0x21, 0x16, 0x8c, 0xe3,
	0xf5, 0x3e, 0xb0, 0xdd, 0xc2, 0x7c, 0x82, 0x3f, 0xf4, 0x89, 0xf8, 0xe5, 0x49, 0x8b, 0x9c, 0xae,
	0x45, 0xcf, 0xcc, 0x07, 0xb7, 0xd6, 0x44, 0xc9, 0x03, 0xa3, 0x1f, 0x1a, 0xd9, 0xa0, 0xfe, 0x57,
	0x2f, 0xb8, 0x14, 0x5f, 0x3f, 0x95, 0xa6, 0x45, 0x15, 0x5c, 0x1f, 0xb9, 0xe3, 0x41, 0xfc, 0xa2,
	0x6b, 0x11, 0xda, 0x5a, 0x7c, 0x9e, 0x8c, 0x92, 0xc7, 0x17, 0xba, 0x0f, 0xcf, 0x8c, 0x83, 0xc1,
	0xaf, 0xdf, 0xc8, 0x89, 0x93, 0x93, 0x65, 0xe8, 0x9e, 0x2e, 0x43, 0xf7, 0xdf, 0x32, 0x74, 0x7f,
	0xae, 0x42, 0xe7, 0x74, 0x15, 0x3a, 0x7f, 0x57, 0xa1, 0xf3, 0xe5, 0x6d, 0x2a, 0x74, 0xd6, 0x30,
	0xcc, 0x65, 0x41, 0xb6, 0xdf, 0xe6, 0xf7, 0x2b, 0xd7, 0xa9, 0x17, 0x15, 0x28, 0xb6, 0xdb, 0x1f,
	0xc1, 0x9b, 0xff, 0x01, 0x00, 0x00, 0xff, 0xff, 0x88, 0x5f, 0xae, 0xfc, 0xd3, 0x02, 0x00, 0x00,
}

func (m *Params) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Params) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Params) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.PacketTimeoutTimestamp != 0 {
		i = encodeVarintParams(dAtA, i, uint64(m.PacketTimeoutTimestamp))
		i--
		dAtA[i] = 0x28
	}
	{
		size, err := m.PacketTimeoutHeight.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintParams(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x22
	if len(m.AuthorizedChannel) > 0 {
		i -= len(m.AuthorizedChannel)
		copy(dAtA[i:], m.AuthorizedChannel)
		i = encodeVarintParams(dAtA, i, uint64(len(m.AuthorizedChannel)))
		i--
		dAtA[i] = 0x1a
	}
	if len(m.EpochIdentifier) > 0 {
		i -= len(m.EpochIdentifier)
		copy(dAtA[i:], m.EpochIdentifier)
		i = encodeVarintParams(dAtA, i, uint64(len(m.EpochIdentifier)))
		i--
		dAtA[i] = 0x12
	}
	if m.Enabled {
		i--
		if m.Enabled {
			dAtA[i] = 1
		} else {
			dAtA[i] = 0
		}
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func encodeVarintParams(dAtA []byte, offset int, v uint64) int {
	offset -= sovParams(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *Params) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Enabled {
		n += 2
	}
	l = len(m.EpochIdentifier)
	if l > 0 {
		n += 1 + l + sovParams(uint64(l))
	}
	l = len(m.AuthorizedChannel)
	if l > 0 {
		n += 1 + l + sovParams(uint64(l))
	}
	l = m.PacketTimeoutHeight.Size()
	n += 1 + l + sovParams(uint64(l))
	if m.PacketTimeoutTimestamp != 0 {
		n += 1 + sovParams(uint64(m.PacketTimeoutTimestamp))
	}
	return n
}

func sovParams(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozParams(x uint64) (n int) {
	return sovParams(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *Params) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowParams
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
			return fmt.Errorf("proto: Params: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Params: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Enabled", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowParams
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
			m.Enabled = bool(v != 0)
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field EpochIdentifier", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowParams
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
				return ErrInvalidLengthParams
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthParams
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.EpochIdentifier = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field AuthorizedChannel", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowParams
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
				return ErrInvalidLengthParams
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthParams
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.AuthorizedChannel = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PacketTimeoutHeight", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowParams
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
				return ErrInvalidLengthParams
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthParams
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.PacketTimeoutHeight.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 5:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field PacketTimeoutTimestamp", wireType)
			}
			m.PacketTimeoutTimestamp = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowParams
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.PacketTimeoutTimestamp |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipParams(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthParams
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
func skipParams(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowParams
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
					return 0, ErrIntOverflowParams
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
					return 0, ErrIntOverflowParams
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
				return 0, ErrInvalidLengthParams
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupParams
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthParams
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthParams        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowParams          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupParams = fmt.Errorf("proto: unexpected end of group")
)
