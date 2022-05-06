package error

import "fmt"

func RecoverExpectedPanic() {
	if err := recover(); err != nil {
		fmt.Println("error recovered from panic:\n", err)
	}
}
