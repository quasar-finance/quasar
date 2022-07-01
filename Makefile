GOMOD := $(shell go list -m)
BUILD_DIR ?= $(CURDIR)/build/
MOCKS_DIR = $(CURDIR)/testutil/mock

# Install & build

mkdirs:
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(MOCKS_DIR)

go-mod:
	go mod tidy
	go mod verify
	go mod download

lint:
	go run github.com/golangci/golangci-lint/cmd/golangci-lint run --timeout=10m

proto-gen:
	scripts/gen_proto

build: mkdirs
	scripts/build build_dev

build-prod: mkdirs
	scripts/build build_with_tags prod

.PHONY: mkdirs go-mod lint proto-gen build build-artifacts

# Testing

PACKAGES_UNIT=$(shell go list ./x/epochs/... ./x/intergamm/... ./x/qbank/... ./x/qoracle/... ./x/orion/keeper/... ./x/orion/types/... | grep -E -v "simapp|e2e" | grep -E -v "x/qoracle/client/cli")

mocks: mkdirs
	mockgen -package=mock -destination=./testutil/mock/ibc_channel_mocks.go $(GOMOD)/x/qoracle/types ChannelKeeper
	mockgen -package=mock -destination=./testutil/mock/ica_mocks.go $(GOMOD)/x/intergamm/types ICAControllerKeeper
	mockgen -package=mock -destination=./testutil/mock/ibc_mocks.go $(GOMOD)/x/intergamm/types IBCTransferKeeper
	mockgen -package=mock -destination=./testutil/mock/ics4_wrapper_mocks.go $(GOMOD)/x/qoracle/types ICS4Wrapper
	mockgen -package=mock -destination=./testutil/mock/ibc_port_mocks.go $(GOMOD)/x/qoracle/types PortKeeper
	mockgen -package=mock -destination=./testutil/mock/ibc_connection_mocks.go $(GOMOD)/x/intergamm/types ConnectionKeeper
	mockgen -package=mock -destination=./testutil/mock/ibc_client_mocks.go $(GOMOD)/x/intergamm/types ClientKeeper

test:
	go test -mod=readonly -v $(PACKAGES_UNIT)

test-path:
	go test -mod=readonly -v $(path)

test-ibc-transfer:
	go test -mod=readonly -v -timeout 99999s demos/ibc-test-framework/ibc_transfer_test.go

test-cover: mkdirs
	go test -mod=readonly -timeout 30m -coverprofile=$(BUILD_DIR)/coverage.txt -covermode=atomic $(PACKAGES_UNIT)

test-simulation:
	ignite chain simulate -v

.PHONY: mocks $(MOCKS_DIR) test test-path test-cover test-simulation

# Documentation

doc-gen:
	scripts/gen_grpc_doc

doc-serve:
	scripts/serve_doc_docker

.PHONY: doc-gen doc-serve

# Run targets

run: proto-gen
	scripts/run

run-silent:
	scripts/run > q.log 2>&1

.PHONY: run run-silent
