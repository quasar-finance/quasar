package osmosis

import (
	"encoding/base64"
	"testing"

	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	"github.com/stretchr/testify/require"
)

// TODO make multitest
func TestParseAck(t *testing.T) {
	var err error
	var ack channeltypes.Acknowledgement
	var bytes []byte
	var resp *gammbalancer.MsgCreateBalancerPoolResponse

	ack = channeltypes.NewResultAcknowledgement([]byte("test"))
	_, err = ParseAck(ack, &gammbalancer.MsgCreateBalancerPoolResponse{})
	require.Error(t, err)

	input := &gammbalancer.MsgCreateBalancerPoolResponse{}
	bytes, err = proto.Marshal(input)
	require.NoError(t, err)
	ack = channeltypes.NewResultAcknowledgement(bytes)
	resp, err = ParseAck(ack, &gammbalancer.MsgCreateBalancerPoolResponse{})
	require.NoError(t, err)
	require.NotNil(t, resp)

	bytes, _ = base64.StdEncoding.DecodeString("Ci0KKy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dDcmVhdGVCYWxhbmNlclBvb2w=")
	ack = channeltypes.NewResultAcknowledgement(bytes)
	resp, err = ParseAck(ack, &gammbalancer.MsgCreateBalancerPoolResponse{})
	require.NoError(t, err)
	require.NotNil(t, resp)
}
