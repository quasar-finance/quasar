package cli

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"strconv"

	"github.com/abag/quasarnode/x/gamm/pool-models/balancer"
	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/client/tx"
	channelutils "github.com/cosmos/ibc-go/v2/modules/core/04-channel/client/utils"
	"github.com/spf13/cobra"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

var _ = strconv.Itoa(0)

func CmdSendIbcCreatePool() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "send-ibc-create-pool [src-port] [src-channel]",
		Short: "Send a ibcCreatePool over IBC",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			creator := clientCtx.GetFromAddress().String()
			srcPort := args[0]
			srcChannel := args[1]

			poolFile, _ := cmd.Flags().GetString(FlagPoolFile)
			if poolFile == "" {
				return fmt.Errorf("must pass in a pool json using the --%s flag", FlagPoolFile)
			}
			pool, err := parseCreatePoolFile(poolFile)
			if err != nil {
				return err
			}
			deposit, err := sdk.ParseCoinsNormalized(pool.InitialDeposit)
			if err != nil {
				return err
			}
			poolAssetCoins, err := sdk.ParseDecCoins(pool.Weights)
			if err != nil {
				return err
			}
			if len(deposit) != len(poolAssetCoins) {
				return errors.New("deposit tokens and token weights should have same length")
			}
			swapFee, err := sdk.NewDecFromStr(pool.SwapFee)
			if err != nil {
				return err
			}
			exitFee, err := sdk.NewDecFromStr(pool.ExitFee)
			if err != nil {
				return err
			}
			poolParams := &balancer.BalancerPoolParams{
				SwapFee: swapFee,
				ExitFee: exitFee,
			}
			var poolAssets []gammtypes.PoolAsset
			for i := 0; i < len(poolAssetCoins); i++ {

				if poolAssetCoins[i].Denom != deposit[i].Denom {
					return errors.New("deposit tokens and token weights should have same denom order")
				}

				poolAssets = append(poolAssets, gammtypes.PoolAsset{
					Weight: poolAssetCoins[i].Amount.RoundInt(),
					Token:  deposit[i],
				})
			}

			// Get the relative timeout timestamp
			timeoutTimestamp, err := cmd.Flags().GetUint64(flagPacketTimeoutTimestamp)
			if err != nil {
				return err
			}
			consensusState, _, _, err := channelutils.QueryLatestConsensusState(clientCtx, srcPort, srcChannel)
			if err != nil {
				return err
			}
			if timeoutTimestamp != 0 {
				timeoutTimestamp = consensusState.GetTimestamp() + timeoutTimestamp
			}

			msg := types.NewMsgSendIbcCreatePool(creator, srcPort, srcChannel, timeoutTimestamp, poolParams, poolAssets, pool.FutureGovernor)
			if err := msg.ValidateBasic(); err != nil {
				return err
			}
			return tx.GenerateOrBroadcastTxCLI(clientCtx, cmd.Flags(), msg)
		},
	}

	cmd.Flags().AddFlagSet(FlagSetCreatePool())
	cmd.Flags().Uint64(flagPacketTimeoutTimestamp, DefaultRelativePacketTimeoutTimestamp, "Packet timeout timestamp in nanoseconds. Default is 10 minutes.")
	flags.AddTxFlagsToCmd(cmd)

	_ = cmd.MarkFlagRequired(FlagPoolFile)

	return cmd
}

func parseCreatePoolFile(poolFile string) (*createPoolInputs, error) {
	pool := createPoolInputs{}

	contents, err := ioutil.ReadFile(poolFile)
	if err != nil {
		return nil, err
	}

	// make exception if unknown field exists
	err = json.Unmarshal(contents, &pool)
	if err != nil {
		return nil, err
	}

	return &pool, nil
}
