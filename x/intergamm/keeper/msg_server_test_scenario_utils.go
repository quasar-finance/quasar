//go:build !prod

package keeper

import (
	"bytes"
	"io"
	"os"
	"reflect"
	"testing"
	"time"

	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

type corpusEntry = struct {
	Parent     string
	Path       string
	Data       []byte
	Values     []any
	Generation int
	IsSeed     bool
}

type testDeps struct{}

func (td testDeps) MatchString(pat, str string) (bool, error)   { return true, nil }
func (td testDeps) StartCPUProfile(w io.Writer) error           { return nil }
func (td testDeps) StopCPUProfile()                             {}
func (td testDeps) WriteProfileTo(string, io.Writer, int) error { return nil }
func (td testDeps) ImportPath() string                          { return "" }
func (td testDeps) StartTestLog(io.Writer)                      {}
func (td testDeps) StopTestLog() error                          { return nil }
func (td testDeps) SetPanicOnExit0(bool)                        {}
func (td testDeps) CheckCorpus([]any, []reflect.Type) error     { return nil }
func (td testDeps) RunFuzzWorker(func(corpusEntry) error) error { return nil }
func (td testDeps) ReadCorpus(string, []reflect.Type) ([]corpusEntry, error) {
	return []corpusEntry{}, nil
}
func (td testDeps) ResetCoverage()    {}
func (td testDeps) SnapshotCoverage() {}
func (td testDeps) CoordinateFuzzing(time.Duration, int64, time.Duration, int64, int, []corpusEntry, []reflect.Type, string, string) error {
	return nil
}

func captureTestOutput(f func() int) (string, int) {
	old := os.Stdout // keep backup of the real stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	ret := f()

	outC := make(chan string)
	// copy the output in a separate goroutine so printing can't block indefinitely
	go func() {
		var buf bytes.Buffer
		_, err := io.Copy(&buf, r)
		if err != nil {
			panic(err)
		}
		outC <- buf.String()
	}()

	// back to normal state
	w.Close()
	os.Stdout = old // restoring the real stdout
	out := <-outC

	return out, ret
}

func runTest(name string, testFunc func(*testing.T)) *types.MsgTestScenarioResponse {
	m := testing.MainStart(testDeps{},
		[]testing.InternalTest{
			{
				name,
				testFunc,
			},
		},
		nil, nil, nil,
	)

	result, exitCode := captureTestOutput(
		func() int {
			return m.Run()
		},
	)

	return &types.MsgTestScenarioResponse{
		Result:   result,
		ExitCode: int64(exitCode),
	}
}
