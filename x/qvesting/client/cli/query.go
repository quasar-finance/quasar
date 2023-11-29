package cli

import (
	"context"
	"fmt"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"github.com/spf13/cobra"
)

// GetQueryCmd returns the cli query commands for this module
func GetQueryCmd() *cobra.Command {
	// Group qvesting queries under a subcommand
	cmd := &cobra.Command{
		Use:                        types.ModuleName,
		Short:                      fmt.Sprintf("Querying commands for the %s module", types.ModuleName),
		DisableFlagParsing:         true,
		SuggestionsMinimumDistance: 2,
		RunE:                       client.ValidateCmd,
	}

	cmd.AddCommand(
		CmdQueryParams(),
		CmdQueryVestingAccounts(),
		CmdQueryQVestingAccounts(),
		CmdQuerySpendableBalances(),
		CmdQuerySpendableSupply(),
		CmdQueryVestingLockedSupply(),
		CmdQueryDelegationLockedSupply(),
		CmdQueryDelegatorLockedSupply(),
		CmdTotalLockedSupply(),
	)

	return cmd
}

func CmdQueryParams() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "params",
		Short: "shows the parameters of the module",
		Args:  cobra.NoArgs,
		RunE: func(cmd *cobra.Command, args []string) error {
			clientCtx := client.GetClientContextFromCmd(cmd)

			queryClient := types.NewQueryClient(clientCtx)

			res, err := queryClient.Params(context.Background(), &types.QueryParamsRequest{})
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}

func CmdQueryVestingAccounts() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "vesting-accounts",
		Short: "shows all the existing vesting accounts in a paginated response",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			pageReq, err := client.ReadPageRequest(cmd.Flags())
			if err != nil {
				return err
			}

			params := &types.QueryVestingAccountsRequest{
				Pagination: pageReq,
			}

			res, err := queryClient.VestingAccounts(ctx, params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)
	flags.AddPaginationFlagsToCmd(cmd, "vesting-accounts")

	return cmd
}

func CmdQueryQVestingAccounts() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "qvesting-accounts",
		Short: "shows the existing vesting accounts created via qvesting module in a paginated response",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			pageReq, err := client.ReadPageRequest(cmd.Flags())
			if err != nil {
				return err
			}

			params := &types.QueryQVestingAccountsRequest{
				Pagination: pageReq,
			}

			res, err := queryClient.QVestingAccounts(ctx, params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)
	flags.AddPaginationFlagsToCmd(cmd, "qvesting-accounts")

	return cmd
}

func CmdQuerySpendableBalances() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "spendable-balances [address]",
		Short: "shows the spendable balances for a given vesting account in a paginated response ",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqAddress := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			pageReq, err := client.ReadPageRequest(cmd.Flags())
			if err != nil {
				return err
			}

			params := &types.QuerySpendableBalancesRequest{
				Address:    reqAddress,
				Pagination: pageReq,
			}

			res, err := queryClient.SpendableBalances(ctx, params)
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)
	flags.AddPaginationFlagsToCmd(cmd, "spendable-balances")

	return cmd
}

func CmdQuerySpendableSupply() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "spendable-supply [denom]",
		Short: "shows the total spendable balances supply across all accounts for a given denom",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqDenom := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			res, err := queryClient.SpendableSupply(ctx, &types.QuerySpendableSupplyRequest{Denom: reqDenom})
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}

func CmdQueryVestingLockedSupply() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "vesting-locked-supply [denom]",
		Short: "shows the total vesting locked supply across all the accounts for a given denom",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqDenom := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			res, err := queryClient.VestingLockedSupply(ctx, &types.QueryVestingLockedSupplyRequest{Denom: reqDenom})
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}

func CmdQueryDelegationLockedSupply() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "delegation-locked-supply",
		Short: "shows the total delegation locked supply across all accounts for the staking denom",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			res, err := queryClient.DelegationLockedSupply(ctx, &types.QueryDelegationLockedSupplyRequest{})
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}

func CmdQueryDelegatorLockedSupply() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "delegator-locked-supply [address]",
		Short: "shows the total delegation locked supply in delegation for a given account and the staking denom",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			reqAddress := args[0]

			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			res, err := queryClient.DelegatorLockedSupply(ctx, &types.QueryDelegatorLockedSupplyRequest{Address: reqAddress})
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}

func CmdTotalLockedSupply() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "total-locked-supply",
		Short: "shows the total vesting and delegation locked supply across all accounts for the staking denom",
		Args:  cobra.ExactArgs(0),
		RunE: func(cmd *cobra.Command, args []string) (err error) {
			clientCtx, err := client.GetClientTxContext(cmd)
			if err != nil {
				return err
			}

			queryClient := types.NewQueryClient(clientCtx)
			ctx := cmd.Context()

			res, err := queryClient.TotalLockedSupply(ctx, &types.QueryTotalLockedSupplyRequest{})
			if err != nil {
				return err
			}

			return clientCtx.PrintProto(res)
		},
	}

	flags.AddQueryFlagsToCmd(cmd)

	return cmd
}
