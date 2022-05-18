package types

import (
	"encoding/base64"
	"testing"

	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
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
	_, err = ParseAck[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse](ack, &gammbalancer.MsgCreateBalancerPool{})
	require.Error(t, err)

	bytes, _ = base64.StdEncoding.DecodeString("Ci0KKy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dDcmVhdGVCYWxhbmNlclBvb2w=")
	ack = channeltypes.NewResultAcknowledgement(bytes)
	resp, err = ParseAck[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse](ack, &gammbalancer.MsgCreateBalancerPool{})
	require.NoError(t, err)
	require.NotNil(t, resp)
}
