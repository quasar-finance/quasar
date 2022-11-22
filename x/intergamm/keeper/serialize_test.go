// TODO do not merge me, should only be used for testing/debugging

package keeper

import (
	"fmt"
	"testing"

	"github.com/cosmos/cosmos-sdk/simapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/gamm/types"
	"github.com/stretchr/testify/assert"
)

func TestSerializeTx(t *testing.T) {

	appCodec := simapp.MakeTestEncodingConfig().Marshaler


	msgs := []sdk.Msg{
		&gammtypes.MsgJoinSwapExternAmountIn{
			Sender:            "counter_party_address",
			PoolId:            1,
			TokenIn:           sdk.Coin{
				Denom:  "uqsr",
				Amount: sdk.NewInt(1000),
			},
			ShareOutMinAmount: sdk.OneInt(),
		},
	}

	data, err := icatypes.SerializeCosmosTx(appCodec, msgs)
	assert.NoError(t, err)


	packet := icatypes.InterchainAccountPacketData{
		Type: icatypes.EXECUTE_TX,
		Data: data,
	}

	packetData := packet.GetBytes()
	fmt.Printf("%v", data)
	fmt.Printf("%v\n", packetData)
	fmt.Println(string(packetData))
}