package keeper

import (
	"encoding/base64"
	"testing"

	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	"github.com/stretchr/testify/require"
)

func mustB64DecodeString(t *testing.T, str string) []byte {
	b, err := base64.StdEncoding.DecodeString("Ci0KKy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dDcmVhdGVCYWxhbmNlclBvb2w=")
	require.NoError(t, err)

	return b
}

func TestParseAck(t *testing.T) {
	testCases := []struct {
		name     string
		ackBytes []byte
		req      *gammbalancer.MsgCreateBalancerPool
		resp     *gammbalancer.MsgCreateBalancerPoolResponse
		errorStr string
	}{
		{
			name:     "valid",
			ackBytes: mustB64DecodeString(t, "Ci0KKy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dDcmVhdGVCYWxhbmNlclBvb2w="),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "",
		},
		{
			name:     "invalid ack bytes",
			ackBytes: []byte("test"),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "cannot unmarshall ICA acknowledgement",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			ack := channeltypes.NewResultAcknowledgement(tc.ackBytes)
			resp := &gammbalancer.MsgCreateBalancerPoolResponse{}
			err := parseAck(ack, tc.req, resp)

			if tc.errorStr != "" {
				require.ErrorContains(t, err, tc.errorStr)
				return
			}

			require.NoError(t, err)
			require.Equal(t, tc.resp, resp)
		})
	}
}
