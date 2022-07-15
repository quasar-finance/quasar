// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: qoracle/osmosis.proto

package types

import (
	fmt "fmt"
	_ "github.com/gogo/protobuf/gogoproto"
	proto "github.com/gogo/protobuf/proto"
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

type OsmosisParams struct {
	ICQParams       IBCParams `protobuf:"bytes,1,opt,name=icq_params,json=icqParams,proto3" json:"icq_params" yaml:"icq_params"`
	EpochIdentifier string    `protobuf:"bytes,2,opt,name=epoch_identifier,json=epochIdentifier,proto3" json:"epoch_identifier,omitempty"`
}

func (m *OsmosisParams) Reset()         { *m = OsmosisParams{} }
func (m *OsmosisParams) String() string { return proto.CompactTextString(m) }
func (*OsmosisParams) ProtoMessage()    {}
func (*OsmosisParams) Descriptor() ([]byte, []int) {
	return fileDescriptor_3c9bfd8b8553c95e, []int{0}
}
func (m *OsmosisParams) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *OsmosisParams) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_OsmosisParams.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *OsmosisParams) XXX_Merge(src proto.Message) {
	xxx_messageInfo_OsmosisParams.Merge(m, src)
}
func (m *OsmosisParams) XXX_Size() int {
	return m.Size()
}
func (m *OsmosisParams) XXX_DiscardUnknown() {
	xxx_messageInfo_OsmosisParams.DiscardUnknown(m)
}

var xxx_messageInfo_OsmosisParams proto.InternalMessageInfo

func (m *OsmosisParams) GetICQParams() IBCParams {
	if m != nil {
		return m.ICQParams
	}
	return IBCParams{}
}

func (m *OsmosisParams) GetEpochIdentifier() string {
	if m != nil {
		return m.EpochIdentifier
	}
	return ""
}

func init() {
	proto.RegisterType((*OsmosisParams)(nil), "abag.quasarnode.qoracle.OsmosisParams")
}

func init() { proto.RegisterFile("qoracle/osmosis.proto", fileDescriptor_3c9bfd8b8553c95e) }

var fileDescriptor_3c9bfd8b8553c95e = []byte{
	// 260 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0x12, 0x2d, 0xcc, 0x2f, 0x4a,
	0x4c, 0xce, 0x49, 0xd5, 0xcf, 0x2f, 0xce, 0xcd, 0x2f, 0xce, 0x2c, 0xd6, 0x2b, 0x28, 0xca, 0x2f,
	0xc9, 0x17, 0x12, 0x4f, 0x4c, 0x4a, 0x4c, 0xd7, 0x2b, 0x2c, 0x4d, 0x2c, 0x4e, 0x2c, 0xca, 0xcb,
	0x4f, 0x49, 0xd5, 0x83, 0x2a, 0x93, 0x12, 0x49, 0xcf, 0x4f, 0xcf, 0x07, 0xab, 0xd1, 0x07, 0xb1,
	0x20, 0xca, 0xa5, 0x04, 0x61, 0xa6, 0x64, 0x26, 0x25, 0x43, 0x84, 0x94, 0x96, 0x32, 0x72, 0xf1,
	0xfa, 0x43, 0xcc, 0x0c, 0x48, 0x2c, 0x4a, 0xcc, 0x2d, 0x16, 0xca, 0xe4, 0xe2, 0xca, 0x4c, 0x2e,
	0x8c, 0x2f, 0x00, 0xf3, 0x24, 0x18, 0x15, 0x18, 0x35, 0xb8, 0x8d, 0x94, 0xf4, 0x70, 0x58, 0xa4,
	0xe7, 0xe9, 0xe4, 0x0c, 0xd1, 0xe7, 0xa4, 0x76, 0xe2, 0x9e, 0x3c, 0xc3, 0xa3, 0x7b, 0xf2, 0x9c,
	0x9e, 0xce, 0x81, 0x10, 0xa1, 0x4f, 0xf7, 0xe4, 0x05, 0x2b, 0x13, 0x73, 0x73, 0xac, 0x94, 0x10,
	0x06, 0x2a, 0x05, 0x71, 0x66, 0x26, 0x17, 0x42, 0xad, 0xd2, 0xe4, 0x12, 0x48, 0x2d, 0xc8, 0x4f,
	0xce, 0x88, 0xcf, 0x4c, 0x49, 0xcd, 0x2b, 0xc9, 0x4c, 0xcb, 0x4c, 0x2d, 0x92, 0x60, 0x52, 0x60,
	0xd4, 0xe0, 0x0c, 0xe2, 0x07, 0x8b, 0x7b, 0xc2, 0x85, 0x9d, 0x5c, 0x4e, 0x3c, 0x92, 0x63, 0xbc,
	0xf0, 0x48, 0x8e, 0xf1, 0xc1, 0x23, 0x39, 0xc6, 0x09, 0x8f, 0xe5, 0x18, 0x2e, 0x3c, 0x96, 0x63,
	0xb8, 0xf1, 0x58, 0x8e, 0x21, 0x4a, 0x2b, 0x3d, 0xb3, 0x24, 0xa3, 0x34, 0x49, 0x2f, 0x39, 0x3f,
	0x57, 0x1f, 0xe4, 0x4a, 0x7d, 0x84, 0x2b, 0xf5, 0x2b, 0xf4, 0x61, 0x3e, 0x2e, 0xa9, 0x2c, 0x48,
	0x2d, 0x4e, 0x62, 0x03, 0x7b, 0xda, 0x18, 0x10, 0x00, 0x00, 0xff, 0xff, 0x33, 0x08, 0xd8, 0xc7,
	0x4f, 0x01, 0x00, 0x00,
}

func (m *OsmosisParams) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *OsmosisParams) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *OsmosisParams) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.EpochIdentifier) > 0 {
		i -= len(m.EpochIdentifier)
		copy(dAtA[i:], m.EpochIdentifier)
		i = encodeVarintOsmosis(dAtA, i, uint64(len(m.EpochIdentifier)))
		i--
		dAtA[i] = 0x12
	}
	{
		size, err := m.ICQParams.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintOsmosis(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0xa
	return len(dAtA) - i, nil
}

func encodeVarintOsmosis(dAtA []byte, offset int, v uint64) int {
	offset -= sovOsmosis(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *OsmosisParams) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = m.ICQParams.Size()
	n += 1 + l + sovOsmosis(uint64(l))
	l = len(m.EpochIdentifier)
	if l > 0 {
		n += 1 + l + sovOsmosis(uint64(l))
	}
	return n
}

func sovOsmosis(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozOsmosis(x uint64) (n int) {
	return sovOsmosis(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *OsmosisParams) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowOsmosis
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
			return fmt.Errorf("proto: OsmosisParams: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: OsmosisParams: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ICQParams", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOsmosis
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
				return ErrInvalidLengthOsmosis
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOsmosis
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.ICQParams.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field EpochIdentifier", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOsmosis
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
				return ErrInvalidLengthOsmosis
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthOsmosis
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.EpochIdentifier = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipOsmosis(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthOsmosis
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
func skipOsmosis(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowOsmosis
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
					return 0, ErrIntOverflowOsmosis
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
					return 0, ErrIntOverflowOsmosis
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
				return 0, ErrInvalidLengthOsmosis
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupOsmosis
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthOsmosis
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthOsmosis        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowOsmosis          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupOsmosis = fmt.Errorf("proto: unexpected end of group")
)
