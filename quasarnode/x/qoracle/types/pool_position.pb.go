// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: qoracle/pool_position.proto

package types

import (
	fmt "fmt"
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

type SortedPools struct {
	ID []uint64 `protobuf:"varint,1,rep,packed,name=ID,proto3" json:"ID,omitempty"`
}

func (m *SortedPools) Reset()         { *m = SortedPools{} }
func (m *SortedPools) String() string { return proto.CompactTextString(m) }
func (*SortedPools) ProtoMessage()    {}
func (*SortedPools) Descriptor() ([]byte, []int) {
	return fileDescriptor_a25a73c923fe54d3, []int{0}
}
func (m *SortedPools) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *SortedPools) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_SortedPools.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *SortedPools) XXX_Merge(src proto.Message) {
	xxx_messageInfo_SortedPools.Merge(m, src)
}
func (m *SortedPools) XXX_Size() int {
	return m.Size()
}
func (m *SortedPools) XXX_DiscardUnknown() {
	xxx_messageInfo_SortedPools.DiscardUnknown(m)
}

var xxx_messageInfo_SortedPools proto.InternalMessageInfo

func (m *SortedPools) GetID() []uint64 {
	if m != nil {
		return m.ID
	}
	return nil
}

type PoolPosition struct {
	APY             uint64 `protobuf:"varint,1,opt,name=aPY,proto3" json:"aPY,omitempty"`
	TVL             uint64 `protobuf:"varint,2,opt,name=tVL,proto3" json:"tVL,omitempty"`
	LastUpdatedTime uint64 `protobuf:"varint,3,opt,name=lastUpdatedTime,proto3" json:"lastUpdatedTime,omitempty"`
	Creator         string `protobuf:"bytes,4,opt,name=creator,proto3" json:"creator,omitempty"`
}

func (m *PoolPosition) Reset()         { *m = PoolPosition{} }
func (m *PoolPosition) String() string { return proto.CompactTextString(m) }
func (*PoolPosition) ProtoMessage()    {}
func (*PoolPosition) Descriptor() ([]byte, []int) {
	return fileDescriptor_a25a73c923fe54d3, []int{1}
}
func (m *PoolPosition) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *PoolPosition) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_PoolPosition.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *PoolPosition) XXX_Merge(src proto.Message) {
	xxx_messageInfo_PoolPosition.Merge(m, src)
}
func (m *PoolPosition) XXX_Size() int {
	return m.Size()
}
func (m *PoolPosition) XXX_DiscardUnknown() {
	xxx_messageInfo_PoolPosition.DiscardUnknown(m)
}

var xxx_messageInfo_PoolPosition proto.InternalMessageInfo

func (m *PoolPosition) GetAPY() uint64 {
	if m != nil {
		return m.APY
	}
	return 0
}

func (m *PoolPosition) GetTVL() uint64 {
	if m != nil {
		return m.TVL
	}
	return 0
}

func (m *PoolPosition) GetLastUpdatedTime() uint64 {
	if m != nil {
		return m.LastUpdatedTime
	}
	return 0
}

func (m *PoolPosition) GetCreator() string {
	if m != nil {
		return m.Creator
	}
	return ""
}

func init() {
	proto.RegisterType((*SortedPools)(nil), "abag.quasarnode.qoracle.SortedPools")
	proto.RegisterType((*PoolPosition)(nil), "abag.quasarnode.qoracle.PoolPosition")
}

func init() { proto.RegisterFile("qoracle/pool_position.proto", fileDescriptor_a25a73c923fe54d3) }

var fileDescriptor_a25a73c923fe54d3 = []byte{
	// 247 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0x92, 0x2e, 0xcc, 0x2f, 0x4a,
	0x4c, 0xce, 0x49, 0xd5, 0x2f, 0xc8, 0xcf, 0xcf, 0x89, 0x2f, 0xc8, 0x2f, 0xce, 0x2c, 0xc9, 0xcc,
	0xcf, 0xd3, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x12, 0x4f, 0x4c, 0x4a, 0x4c, 0xd7, 0x2b, 0x2c,
	0x4d, 0x2c, 0x4e, 0x2c, 0xca, 0xcb, 0x4f, 0x49, 0xd5, 0x83, 0x2a, 0x56, 0x92, 0xe5, 0xe2, 0x0e,
	0xce, 0x2f, 0x2a, 0x49, 0x4d, 0x09, 0xc8, 0xcf, 0xcf, 0x29, 0x16, 0xe2, 0xe3, 0x62, 0xf2, 0x74,
	0x91, 0x60, 0x54, 0x60, 0xd6, 0x60, 0x09, 0x62, 0xf2, 0x74, 0x51, 0x2a, 0xe3, 0xe2, 0x01, 0x49,
	0x04, 0x40, 0x4d, 0x13, 0x12, 0xe0, 0x62, 0x4e, 0x0c, 0x88, 0x94, 0x60, 0x54, 0x60, 0xd4, 0x60,
	0x09, 0x02, 0x31, 0x41, 0x22, 0x25, 0x61, 0x3e, 0x12, 0x4c, 0x10, 0x91, 0x92, 0x30, 0x1f, 0x21,
	0x0d, 0x2e, 0xfe, 0x9c, 0xc4, 0xe2, 0x92, 0xd0, 0x82, 0x94, 0xc4, 0x92, 0xd4, 0x94, 0x90, 0xcc,
	0xdc, 0x54, 0x09, 0x66, 0xb0, 0x2c, 0xba, 0xb0, 0x90, 0x04, 0x17, 0x7b, 0x72, 0x51, 0x6a, 0x62,
	0x49, 0x7e, 0x91, 0x04, 0x8b, 0x02, 0xa3, 0x06, 0x67, 0x10, 0x8c, 0xeb, 0xe4, 0x72, 0xe2, 0x91,
	0x1c, 0xe3, 0x85, 0x47, 0x72, 0x8c, 0x0f, 0x1e, 0xc9, 0x31, 0x4e, 0x78, 0x2c, 0xc7, 0x70, 0xe1,
	0xb1, 0x1c, 0xc3, 0x8d, 0xc7, 0x72, 0x0c, 0x51, 0x5a, 0xe9, 0x99, 0x25, 0x19, 0xa5, 0x49, 0x7a,
	0xc9, 0xf9, 0xb9, 0xfa, 0x20, 0x4f, 0xe9, 0x23, 0x3c, 0xa5, 0x5f, 0xa1, 0x0f, 0x0b, 0x83, 0x92,
	0xca, 0x82, 0xd4, 0xe2, 0x24, 0x36, 0xb0, 0xe7, 0x8d, 0x01, 0x01, 0x00, 0x00, 0xff, 0xff, 0x7b,
	0xb5, 0x59, 0xed, 0x1b, 0x01, 0x00, 0x00,
}

func (m *SortedPools) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *SortedPools) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *SortedPools) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.ID) > 0 {
		dAtA2 := make([]byte, len(m.ID)*10)
		var j1 int
		for _, num := range m.ID {
			for num >= 1<<7 {
				dAtA2[j1] = uint8(uint64(num)&0x7f | 0x80)
				num >>= 7
				j1++
			}
			dAtA2[j1] = uint8(num)
			j1++
		}
		i -= j1
		copy(dAtA[i:], dAtA2[:j1])
		i = encodeVarintPoolPosition(dAtA, i, uint64(j1))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *PoolPosition) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *PoolPosition) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *PoolPosition) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Creator) > 0 {
		i -= len(m.Creator)
		copy(dAtA[i:], m.Creator)
		i = encodeVarintPoolPosition(dAtA, i, uint64(len(m.Creator)))
		i--
		dAtA[i] = 0x22
	}
	if m.LastUpdatedTime != 0 {
		i = encodeVarintPoolPosition(dAtA, i, uint64(m.LastUpdatedTime))
		i--
		dAtA[i] = 0x18
	}
	if m.TVL != 0 {
		i = encodeVarintPoolPosition(dAtA, i, uint64(m.TVL))
		i--
		dAtA[i] = 0x10
	}
	if m.APY != 0 {
		i = encodeVarintPoolPosition(dAtA, i, uint64(m.APY))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func encodeVarintPoolPosition(dAtA []byte, offset int, v uint64) int {
	offset -= sovPoolPosition(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *SortedPools) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if len(m.ID) > 0 {
		l = 0
		for _, e := range m.ID {
			l += sovPoolPosition(uint64(e))
		}
		n += 1 + sovPoolPosition(uint64(l)) + l
	}
	return n
}

func (m *PoolPosition) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.APY != 0 {
		n += 1 + sovPoolPosition(uint64(m.APY))
	}
	if m.TVL != 0 {
		n += 1 + sovPoolPosition(uint64(m.TVL))
	}
	if m.LastUpdatedTime != 0 {
		n += 1 + sovPoolPosition(uint64(m.LastUpdatedTime))
	}
	l = len(m.Creator)
	if l > 0 {
		n += 1 + l + sovPoolPosition(uint64(l))
	}
	return n
}

func sovPoolPosition(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozPoolPosition(x uint64) (n int) {
	return sovPoolPosition(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *SortedPools) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPoolPosition
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
			return fmt.Errorf("proto: SortedPools: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: SortedPools: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType == 0 {
				var v uint64
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowPoolPosition
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					v |= uint64(b&0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				m.ID = append(m.ID, v)
			} else if wireType == 2 {
				var packedLen int
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowPoolPosition
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					packedLen |= int(b&0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				if packedLen < 0 {
					return ErrInvalidLengthPoolPosition
				}
				postIndex := iNdEx + packedLen
				if postIndex < 0 {
					return ErrInvalidLengthPoolPosition
				}
				if postIndex > l {
					return io.ErrUnexpectedEOF
				}
				var elementCount int
				var count int
				for _, integer := range dAtA[iNdEx:postIndex] {
					if integer < 128 {
						count++
					}
				}
				elementCount = count
				if elementCount != 0 && len(m.ID) == 0 {
					m.ID = make([]uint64, 0, elementCount)
				}
				for iNdEx < postIndex {
					var v uint64
					for shift := uint(0); ; shift += 7 {
						if shift >= 64 {
							return ErrIntOverflowPoolPosition
						}
						if iNdEx >= l {
							return io.ErrUnexpectedEOF
						}
						b := dAtA[iNdEx]
						iNdEx++
						v |= uint64(b&0x7F) << shift
						if b < 0x80 {
							break
						}
					}
					m.ID = append(m.ID, v)
				}
			} else {
				return fmt.Errorf("proto: wrong wireType = %d for field ID", wireType)
			}
		default:
			iNdEx = preIndex
			skippy, err := skipPoolPosition(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthPoolPosition
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
func (m *PoolPosition) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPoolPosition
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
			return fmt.Errorf("proto: PoolPosition: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: PoolPosition: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field APY", wireType)
			}
			m.APY = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPoolPosition
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.APY |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TVL", wireType)
			}
			m.TVL = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPoolPosition
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TVL |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field LastUpdatedTime", wireType)
			}
			m.LastUpdatedTime = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPoolPosition
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.LastUpdatedTime |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Creator", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPoolPosition
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
				return ErrInvalidLengthPoolPosition
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthPoolPosition
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Creator = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPoolPosition(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthPoolPosition
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
func skipPoolPosition(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowPoolPosition
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
					return 0, ErrIntOverflowPoolPosition
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
					return 0, ErrIntOverflowPoolPosition
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
				return 0, ErrInvalidLengthPoolPosition
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupPoolPosition
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthPoolPosition
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthPoolPosition        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowPoolPosition          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupPoolPosition = fmt.Errorf("proto: unexpected end of group")
)
