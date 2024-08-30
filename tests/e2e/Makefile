e2e-help:
	@echo "e2e subcommands"
	@echo ""
	@echo "Usage:"
	@echo "  make e2e-[command]"
	@echo ""
	@echo "Available Commands:"
	@echo "  test                Run tests in the specified folders serially, if CASES is defined; otherwise, run all."
	@echo "  test-parallel       Run tests in the specified folders in parallel, if CASES is defined; otherwise, run all."

e2e: e2e-help

# Find all folders inside ./cases, excluding ones that start with an underscore
TEST_FOLDERS=$(shell find ./tests/e2e/cases -mindepth 1 -maxdepth 1 -type d \( -name "[!_]*" \))

# Run tests in the specified folders serially, if CASES is defined; otherwise, run all.
e2e-test:
ifdef CASES
    # Loop through each folder specified in CASES and run the tests
	@for folder in $(filter $(CASES), $(TEST_FOLDERS)); do \
		(echo "\nRunning tests in directory: $$folder"; \
		cd $$folder && go test ./... -v -timeout 99999s); \
	done
else
    # Loop through each folder in TEST_FOLDERS and run the tests
	@for folder in $(TEST_FOLDERS); do \
		(echo "\nRunning tests in directory: $$folder"; \
		 cd $$folder && go test ./... -v -timeout 99999s); \
	done
endif

# Run tests in the specified folders in parallel, if CASES is defined; otherwise, run all.
e2e-test-parallel:
ifdef CASES
	@for folder in $(filter $(CASES), $(TEST_FOLDERS)); do \
		echo "\nRunning tests in directory: $$folder"; \
		cd $$folder && go test ./... -timeout 99999s & \
	done; wait
else
	@for folder in $(TEST_FOLDERS); do \
		echo "\nRunning tests in directory: $$folder"; \
		cd $$folder && go test ./... -timeout 99999s & \
	done; wait
endif

