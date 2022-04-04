go.mod:
	go mod tidy && go mod verify

test:
	go test -v ./x/qbank/...

test_simulation:
	starport chain simulate -v

run:
	./run.sh

.PHONY: go.mod test test_simulation run
