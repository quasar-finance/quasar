go.mod:
	go mod tidy && go mod verify

test_path:
	go test -v $(path)

test:
	@$(MAKE) test_path path="./x/qbank/... ./x/epochs/... "

test_simulation:
	starport chain simulate -v

run:
	./run.sh

run_silent:
	./run.sh > q.log 2>&1

.PHONY: go.mod test_path test test_simulation run run_silent
