package types

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestCreateOsmosisSpotPriceKey(t *testing.T) {
	key := CreateOsmosisSpotPriceKey(26, "uatom", "uosmo")
	require.Equal(t, "\x00\x00\x00\x00\x00\x00\x00\x1A#uatom#uosmo", string(key))
}
