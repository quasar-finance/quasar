TEST_FOLDERS=$(shell find ./cases -mindepth 1 -maxdepth 1 -type d \( -name "[!_]*" \))

test-e2e:
ifdef CASES
	@for folder in $(filter $(CASES), $(TEST_FOLDERS)); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -v); \
	done
else
	@for folder in $(TEST_FOLDERS); do \
		(echo "\nRunning tests in directory: $$folder"; cd $$folder && go test ./... -v); \
	done
endif
