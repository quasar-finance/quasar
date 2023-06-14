package helpers

// #cgo LDFLAGS: -L../random_generator/target/release -lrandom_generator
// #include <stdint.h>
// #include <stdlib.h>
// #include <string.h>
// extern const char* generate_random_test_case(unsigned long long seed, size_t count, unsigned long long min, unsigned long long max);
import "C"
import (
	"encoding/json"
	"fmt"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"unsafe"
)

func GenerateTestCases(seed, count, min, max int64) (testsuite.TestCases, error) {
	cResult := C.generate_random_test_case(C.ulonglong(seed), C.size_t(count), C.ulonglong(min), C.ulonglong(max))
	defer C.free(unsafe.Pointer(cResult))

	result := C.GoString(cResult)

	var data testsuite.WasmTestCasesForGenerator
	err := json.Unmarshal([]byte(result), &data)
	if err != nil {
		fmt.Println("Failed to parse JSON:", err)
		return nil, err
	}

	testCases, err := data.ConvertToTestCases()
	if err != nil {
		return nil, err
	}

	return testCases, nil
}
