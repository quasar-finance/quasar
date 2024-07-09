package cli_test

// todo uncomment them again once sdk issue is fixed

//func setupNetwork(t *testing.T) *network.Network {
//	t.Helper()
//	n := network.New(t)
//	_, err := n.WaitForHeight(1)
//	require.NoError(t, err)
//
//	return n
//}

//func TestGetCmdCurrentEpoch(t *testing.T) {
//	n := setupNetwork(t)
//
//	clientCtx := n.Validators[0].ClientCtx
//	common := []string{
//		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
//	}
//	testCases := []struct {
//		name       string
//		identifier string
//		args       []string
//		expectErr  bool
//		resp       types.QueryCurrentEpochResponse
//	}{
//		{
//			"query minutely epoch number",
//			"minute",
//			common,
//			false,
//			types.QueryCurrentEpochResponse{
//				CurrentEpoch: int64(1),
//			},
//		},
//		{
//			"query daily epoch number",
//			"day",
//			common,
//			false,
//			types.QueryCurrentEpochResponse{
//				CurrentEpoch: int64(1),
//			},
//		},
//		{
//			"query weekly epoch number",
//			"week",
//			common,
//			false,
//			types.QueryCurrentEpochResponse{
//				CurrentEpoch: int64(1),
//			},
//		},
//		{
//			"query unavailable epoch number",
//			"unavailable",
//			common,
//			true,
//			types.QueryCurrentEpochResponse{},
//		},
//	}
//
//	for _, tc := range testCases {
//		tc := tc
//
//		t.Run(tc.name, func(t *testing.T) {
//			cmd := cli.GetCmdCurrentEpoch()
//			args := []string{
//				tc.identifier,
//			}
//			args = append(args, tc.args...)
//			out, err := clitestutil.ExecTestCLICmd(clientCtx, cmd, args)
//			if tc.expectErr {
//				require.Error(t, err)
//			} else {
//				require.NoError(t, err, out.String())
//
//				var actualResp types.QueryCurrentEpochResponse
//				err := clientCtx.Codec.UnmarshalJSON(out.Bytes(), &actualResp)
//				require.NoError(t, err)
//				require.Equal(t, tc.resp, actualResp)
//			}
//		})
//	}
//}
//
//func TestGetCmdEpochsInfos(t *testing.T) {
//	var err error
//	n := setupNetwork(t)
//	clientCtx := n.Validators[0].ClientCtx
//	common := []string{
//		fmt.Sprintf("--%s=json", tmcli.OutputFlag),
//	}
//	cmd := cli.GetCmdEpochsInfos()
//
//	out, err := clitestutil.ExecTestCLICmd(clientCtx, cmd, common)
//	require.NoError(t, err, out.String())
//
//	var resp types.QueryEpochsInfoResponse
//	err = clientCtx.Codec.UnmarshalJSON(out.Bytes(), &resp)
//	require.NoError(t, err)
//	require.Equal(t, 4, len(resp.Epochs))
//}
