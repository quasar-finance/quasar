go.mod:
	go mod tidy && go mod verify

test_path:
	go test -v $(path)

test:
	@$(MAKE) test_path path="./x/qbank/..."

test_simulation:
	starport chain simulate -v

run:
	./run.sh

.PHONY: go.mod test_file test test_simulation run
