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

$(BUILD_DIR)/quasarnoded: mkdirs
	scripts/build build_dev

build-artifacts: mkdirs
	scripts/build build_artifacts

.PHONY: mkdirs go-mod lint proto-gen build build-artifacts

# Testing

PACKAGES_UNIT=$(shell go list ./x/epochs/... ./x/intergamm/... ./x/qbank/... ./x/qoracle/... ./x/orion/keeper/... ./x/orion/types/... | grep -E -v "simapp|e2e" | grep -E -v "x/qoracle/client/cli")

mocks: mkdirs
	mockgen -package=mock -destination=./testutil/mock/ibc_channel_mocks.go $(GOMOD)/x/intergamm/types ChannelKeeper
	mockgen -package=mock -destination=./testutil/mock/ica_mocks.go $(GOMOD)/x/intergamm/types ICAControllerKeeper
	mockgen -package=mock -destination=./testutil/mock/ibc_mocks.go $(GOMOD)/x/intergamm/types IBCTransferKeeper

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

###############################################################################
###                                Localnet                                 ###
###############################################################################
# TODO move to script

NUM_NODES ?= 4
LOCAL_TESTNET_RUN_DIR=$(CURDIR)/local_testnet_run-$(NUM_NODES)nodes/

build-docker-image:
	$(MAKE) -C docker/local_testnet quasarnoded-env

$(LOCAL_TESTNET_RUN_DIR):
	mkdir -p "$@"

local-testnet-init: $(BUILD_DIR)/quasarnoded $(LOCAL_TESTNET_RUN_DIR) build-docker-image
	cp $(BUILD_DIR)/quasarnoded $(LOCAL_TESTNET_RUN_DIR)
	@if ! [ -f $(LOCAL_TESTNET_RUN_DIR)/node0/quasarnoded/config/genesis.json ]; \
	then $(BUILD_DIR)/quasarnoded testnet --v $(NUM_NODES) -o $(LOCAL_TESTNET_RUN_DIR) --starting-ip-address 192.168.10.2 --keyring-backend=test ; \
	fi

# Run testnet locally
local-testnet-start: # local-testnet-stop
	sudo docker-compose -f $(LOCAL_TESTNET_RUN_DIR)/docker-compose.yml up -d

.PHONY: build-docker-image local-testnet-init local-testnet-start

check_defined = $(strip $(foreach 1,$1, $(call __check_defined,$1)))
__check_defined = $(if $(value $1),, $(error Undefined $1))

# Execute quasar command in a running service
local-testnet-exec-quasar:
	@:$(call check_defined, node cmd)
	sudo docker-compose -f $(LOCAL_TESTNET_RUN_DIR)/docker-compose.yml exec quasarnode$(node) run.sh '$(cmd)'

# Execute bash command in a running service
local-testnet-exec-bash:
	@:$(call check_defined, node cmd)
	sudo docker-compose -f $(LOCAL_TESTNET_RUN_DIR)/docker-compose.yml exec quasarnode$(node) '$(cmd)'

# Run a service and execute a quasar command
local-testnet-run-quasar:
	@:$(call check_defined, node cmd)
	sudo docker-compose -f $(LOCAL_TESTNET_RUN_DIR)/docker-compose.yml run --rm quasarnode$(node) '$(cmd)'

# Run a service and execute a bash command
local-testnet-run-bash:
	@:$(call check_defined, node cmd)
	sudo docker-compose -f $(LOCAL_TESTNET_RUN_DIR)/docker-compose.yml run --rm --entrypoint 'bash -c' quasarnode$(node) '$(cmd)'

.PHONY: local-testnet-exec-quasar local-testnet-exec-bash local-testnet-run-quasar local-testnet-run-bash

# Stop testnet
local-testnet-stop:
	-sudo docker-compose -f $(LOCAL_TESTNET_RUN_DIR)/docker-compose.yml down

clean-local-testnet: local-testnet-stop
	-rm -rf $(LOCAL_TESTNET_RUN_DIR)/

clean-all-local-testnets: local-testnet-stop
	-rm -rf $(CURDIR)/local_testnet_run-*nodes/

.PHONY: local-testnet-stop clean-local-testnet clean-all-local-testnets
