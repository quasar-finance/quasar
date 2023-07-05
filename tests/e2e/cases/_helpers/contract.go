package helpers

import (
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"log"
	"strconv"
	"strings"
)

func ParseTrappedError(trappedError testsuite.ContractTrappedErrorsData) (uint64, string) {
	var seqError uint64
	var channelIdError string
	for key := range trappedError.Data.TrappedErrors {
		splitKey := strings.Split(key, "-")
		seqTemp, err := strconv.ParseInt(splitKey[0], 10, 64)
		if err != nil {
			log.Fatalf("Failed to convert seq to int64: %v", err)
		}
		seqError = uint64(seqTemp)
		channelIdError = splitKey[1] + "-" + splitKey[2]
		break
	}
	return seqError, channelIdError
}
