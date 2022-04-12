package events

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

func AssertEventEmitted(t *testing.T, ctx sdk.Context, eventType string) {
	for _, e := range ctx.EventManager().Events() {
		if e.Type == eventType {
			return
		}
	}
	t.Fatalf("expected event '%s' not found", eventType)
}
