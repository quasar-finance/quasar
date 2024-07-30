package types_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/cmd/quasard/cmd"
	"github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

func TestDeconstructDenom(t *testing.T) {
	cmd.InitTestConfig()
	for _, tc := range []struct {
		desc             string
		denom            string
		expectedSubdenom string
		err              error
	}{
		{
			desc:  "empty is invalid",
			denom: "",
			err:   types.ErrInvalidDenom,
		},
		{
			desc:             "normal",
			denom:            "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
			expectedSubdenom: "bitcoin",
		},
		{
			desc:             "multiple slashes in subdenom",
			denom:            "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin/1",
			expectedSubdenom: "bitcoin/1",
		},
		{
			desc:             "no subdenom",
			denom:            "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/",
			expectedSubdenom: "",
		},
		{
			desc:  "incorrect prefix",
			denom: "ibc/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
			err:   types.ErrInvalidDenom,
		},
		{
			desc:             "subdenom of only slashes",
			denom:            "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/////",
			expectedSubdenom: "////",
		},
		{
			desc:  "too long name",
			denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/adsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsf",
			err:   types.ErrInvalidDenom,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			expectedCreator := "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
			creator, subdenom, err := types.DeconstructDenom(tc.denom)
			if tc.err != nil {
				require.ErrorContains(t, err, tc.err.Error())
			} else {
				require.NoError(t, err)
				require.Equal(t, expectedCreator, creator)
				require.Equal(t, tc.expectedSubdenom, subdenom)
			}
		})
	}
}

func TestGetTokenDenom(t *testing.T) {
	cmd.InitTestConfig()
	for _, tc := range []struct {
		desc     string
		creator  string
		subdenom string
		valid    bool
	}{
		{
			desc:     "normal",
			creator:  "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
			subdenom: "bitcoin",
			valid:    true,
		},
		{
			desc:     "multiple slashes in subdenom",
			creator:  "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
			subdenom: "bitcoin/1",
			valid:    true,
		},
		{
			desc:     "no subdenom",
			creator:  "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
			subdenom: "",
			valid:    true,
		},
		{
			desc:     "subdenom of only slashes",
			creator:  "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
			subdenom: "/////",
			valid:    true,
		},
		{
			desc:     "too long name",
			creator:  "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
			subdenom: "adsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsfadsf",
			valid:    false,
		},
		{
			desc:     "subdenom is exactly max length",
			creator:  "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
			subdenom: "bitcoinfsadfsdfeadfsafwefsefsefsdfsdafasefsf",
			valid:    true,
		},
		{
			desc:     "creator is exactly max length",
			creator:  "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwecjhgjhgkhjklhkjhkjhgjhgjgjghelu",
			subdenom: "bitcoin",
			valid:    true,
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			_, err := types.GetTokenDenom(tc.creator, tc.subdenom)
			if tc.valid {
				require.NoError(t, err)
			} else {
				require.Error(t, err)
			}
		})
	}
}
