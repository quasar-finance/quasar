package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	// "github.com/gogo/protobuf/proto"
	"github.com/cosmos/gogoproto/proto" // SDK47
	balancerpool "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*proto.Message)(nil), &balancerpool.Pool{})
	// Does not need for the updated version of the interfaces for the
	// SDK47. - I think so. Recheck. Otherwise it will give error
	// panic(fmt.Errorf("error unzipping file description for MsgService %s", sd.ServiceName))
	// in the go/pkg/mod/github.com/cosmos/cosmos-sdk@v0.47.5/types/msgservice/msg_service.go:24
	// RegisterMsgServiceDesc(registry codectypes.InterfaceRegistry, sd *grpc.ServiceDesc)
	// msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

var (
	Amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewProtoCodec(cdctypes.NewInterfaceRegistry())
)
