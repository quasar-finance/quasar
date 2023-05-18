package e2e

import (
	"fmt"
	testSuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/stretchr/testify/suite"
	"testing"
)

type TestE2eTestBuilderSuite struct {
	*testSuite.E2eTestBuilder
	suite.Suite
}

func TestE2eTestBuilder(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	s := testSuite.NewE2eTestBuilder(t)

	accounts := testSuite.AccountsNew{}
	s.AddChain(testSuite.QuasarChain, accounts, 1, 1, true)

	b := &TestE2eTestBuilderSuite{
		E2eTestBuilder: s.Build(),
	}
	suite.Run(t, b)

	c, found := b.Chains.GetChain("quasar")
	if found {
		newConrtacts := []*testSuite.Contract{
			testSuite.NewContract("test", "test", 1),
			testSuite.NewContract("test", "test", 2),
		}

		c.SetContracts(newConrtacts)
	}

	d, found := b.Chains.GetChain("quasar")
	if found {
		for _, ct := range d.GetContracts() {
			fmt.Println(ct.GetCodeID())
		}
	}

}
