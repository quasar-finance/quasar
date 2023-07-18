# Find all folders inside ./cases, excluding ones that start with an underscore
TEST_FOLDERS=$(shell find ./cases -mindepth 1 -maxdepth 1 -type d \( -name "[!_]*" \))

# Run tests in the specified folders serially, if CASES is defined; otherwise, run all.
test-e2e:
ifdef CASES
	# Loop through each folder specified in CASES and run the tests
	@status=0; \
	for folder in $(filter $(CASES), $(TEST_FOLDERS)); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -v -timeout 99999s) || status=$$?; \
	done; \
	exit $$status
else
	# Loop through each folder in TEST_FOLDERS and run the tests
	@status=0; \
	for folder in $(TEST_FOLDERS); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -v -timeout 99999s) || status=$$?; \
	done; \
	exit $$status
endif

# Run tests in the specified folders in parallel, if CASES is defined; otherwise, run all.
test-e2e-parallel:
ifdef CASES
	# Loop through each folder specified in CASES and run the tests in the background
	@status=0; \
	for folder in $(filter $(CASES), $(TEST_FOLDERS)); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -timeout 99999s) || status=$$?; \
	done; wait; exit $$status # Wait for all background jobs to finish
else
	# Loop through each folder in TEST_FOLDERS and run the tests in the background
	@status=0; \
	for folder in $(TEST_FOLDERS); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -timeout 99999s) || status=$$?; \
	done; wait; exit $$status # Wait for all background jobs to finish
endif
