package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

type IntergammHooks interface {
	OnIcaAcknowledgement(ctx sdk.Context)
}

// var _ IntergammHooks = MultiIntergammHooks{}

// // combine multiple gamm hooks, all hook functions are run in array sequence.
// type MultiIntergammHooks []IntergammHooks

// func NewMultiIntergammHooks(hooks ...IntergammHooks) MultiIntergammHooks {
// 	return hooks
// }

// // OnIcaAcknowledgement is called when epoch is going to be ended, epochNumber is the number of epoch that is ending.
// func (h MultiIntergammHooks) OnIcaAcknowledgement(ctx sdk.Context) {
// 	for i := range h {
// 		h[i].OnIcaAcknowledgement(ctx)
// 	}
// }
