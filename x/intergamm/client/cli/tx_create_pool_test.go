package cli

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestParseInputData(t *testing.T) {
	var err error
	json := []byte(`
{
	"connection_id": "connection-0",
	"timeout_timestamp": "42",
	"weights": "4uatom,4osmo,2uakt",
	"initial_deposit": "100uatom,5osmo,20uakt",
	"swap_fee": "0.01",
	"exit_fee": "0.01",
	"future_governor": "168h"
}
	`)

	addr := sample.AccAddress().String()
	inputData, err := parseInputData(json)
	require.NoError(t, err)

	msg, err := inputDataToMessage(addr, inputData)
	require.NoError(t, err)
	require.Equal(t, addr, msg.Creator)
}
