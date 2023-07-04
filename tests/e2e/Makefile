# Find all folders inside ./cases, excluding ones that start with an underscore
TEST_FOLDERS=$(shell find ./cases -mindepth 1 -maxdepth 1 -type d \( -name "[!_]*" \))

# Run tests in the specified folders serially, if CASES is defined; otherwise, run all.
test-e2e:
ifdef CASES
	# Loop through each folder specified in CASES and run the tests
	@for folder in $(filter $(CASES), $(TEST_FOLDERS)); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -v -timeout 99999s); \
	done
else
	# Loop through each folder in TEST_FOLDERS and run the tests
	@for folder in $(TEST_FOLDERS); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -v -timeout 99999s); \
	done
endif

# Run tests in the specified folders in parallel, if CASES is defined; otherwise, run all.
test-e2e-parallel:
ifdef CASES
	# Loop through each folder specified in CASES and run the tests in the background
	@for folder in $(filter $(CASES), $(TEST_FOLDERS)); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -timeout 99999s) & \
	done; wait # Wait for all background jobs to finish
else
	# Loop through each folder in TEST_FOLDERS and run the tests in the background
	@for folder in $(TEST_FOLDERS); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -timeout 99999s) & \
	done; wait # Wait for all background jobs to finish
endif
