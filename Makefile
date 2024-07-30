#!/usr/bin/make -f

include scripts/makefiles/build.mk
include scripts/makefiles/docker.mk
include scripts/makefiles/lint.mk
include scripts/makefiles/proto.mk
include scripts/makefiles/test.mk
include tests/e2e/Makefile

.DEFAULT_GOAL := help
help:
	@echo "Available top-level commands:"
	@echo ""
	@echo "Usage:"
	@echo "    make [command]"
	@echo ""
	@echo "  make build                 Build quasard binary"
	@echo "  make build-help            Show available build commands"
	@echo "  make docker			    Show available docker commands"
	@echo "  make e2e                   Show available e2e commands"
	@echo "  make install               Install quasard binary"
	@echo "  make lint                  Show available lint commands"
	@echo "  make test                  Show available test commands"
	@echo ""
	@echo "Run 'make [subcommand]' to see the available commands for each subcommand."

VERSION := $(shell echo $(shell git describe --tags) | sed 's/^v//')
COMMIT := $(shell git log -1 --format='%H')
LEDGER_ENABLED ?= true
SDK_PACK := $(shell go list -m github.com/cosmos/cosmos-sdk | sed  's/ /\@/g')
GO_VERSION := $(shell cat go.mod | grep -E 'go [0-9].[0-9]+' | cut -d ' ' -f 2)
DOCKER := $(shell which docker)
GOMOD := $(shell go list -m)
GO_MODULE := $(shell cat go.mod | grep "module " | cut -d ' ' -f 2)
GO_MAJOR_VERSION = $(shell go version | cut -c 14- | cut -d' ' -f1 | cut -d'.' -f1)
GO_MINOR_VERSION = $(shell go version | cut -c 14- | cut -d' ' -f1 | cut -d'.' -f2)
# minimum supported Go version
GO_MINIMUM_MAJOR_VERSION = $(shell cat go.mod | grep -E 'go [0-9].[0-9]+' | cut -d ' ' -f2 | cut -d'.' -f1)
GO_MINIMUM_MINOR_VERSION = $(shell cat go.mod | grep -E 'go [0-9].[0-9]+' | cut -d ' ' -f2 | cut -d'.' -f2)

BUILDDIR ?= $(CURDIR)/build
MOCKSDIR = $(CURDIR)/testutil/mock

export GO111MODULE = on

# process build tags

build_tags = netgo
ifeq ($(LEDGER_ENABLED),true)
  ifeq ($(OS),Windows_NT)
    GCCEXE = $(shell where gcc.exe 2> NUL)
    ifeq ($(GCCEXE),)
      $(error gcc.exe not installed for ledger support, please install or set LEDGER_ENABLED=false)
    else
      build_tags += ledger
    endif
  else
    UNAME_S = $(shell uname -s)
    ifeq ($(UNAME_S),OpenBSD)
      $(warning OpenBSD detected, disabling ledger support (https://github.com/cosmos/cosmos-sdk/issues/1988))
    else
      GCC = $(shell command -v gcc 2> /dev/null)
      ifeq ($(GCC),)
        $(error gcc not installed for ledger support, please install or set LEDGER_ENABLED=false)
      else
        build_tags += ledger
      endif
    endif
  endif
endif

ifeq (cleveldb,$(findstring cleveldb,$(QUASAR_BUILD_OPTIONS)))
  build_tags += gcc
else ifeq (rocksdb,$(findstring rocksdb,$(QUASAR_BUILD_OPTIONS)))
  build_tags += gcc
endif
build_tags += $(BUILD_TAGS)
build_tags_debug += $(BUILD_TAGS)
build_tags := $(strip $(build_tags))

whitespace :=
whitespace += $(whitespace)
comma := ,
build_tags_comma_sep := $(subst $(whitespace),$(comma),$(build_tags))

# process linker flags

ldflags = -X github.com/cosmos/cosmos-sdk/version.Name=quasar \
		  -X github.com/cosmos/cosmos-sdk/version.AppName=quasard \
		  -X github.com/cosmos/cosmos-sdk/version.Version=$(VERSION) \
		  -X github.com/cosmos/cosmos-sdk/version.Commit=$(COMMIT) \
		  -X "github.com/cosmos/cosmos-sdk/version.BuildTags=$(build_tags_comma_sep)"

ifeq (cleveldb,$(findstring cleveldb,$(QUASAR_BUILD_OPTIONS)))
  ldflags += -X github.com/cosmos/cosmos-sdk/types.DBBackend=cleveldb
else ifeq (rocksdb,$(findstring rocksdb,$(QUASAR_BUILD_OPTIONS)))
  ldflags += -X github.com/cosmos/cosmos-sdk/types.DBBackend=rocksdb
endif

ldflags-debug := $(ldflags)

ifeq (,$(findstring nostrip,$(QUASAR_BUILD_OPTIONS)))
  ldflags += -w -s
endif

ifeq ($(LINK_STATICALLY),true)
	ldflags += -linkmode=external -extldflags "-Wl,-z,muldefs -static"
	ldflags-debug += -linkmode=external -extldflags "-Wl,-z,muldefs -static"
endif

ldflags += $(LDFLAGS)
ldflags-debug += $(LDFLAGS)

ldflags := $(strip $(ldflags))

BUILD_FLAGS := -tags "$(build_tags)" -ldflags '$(ldflags)'
BUILD_FLAGS_DEBUG := -tags "$(build_tags_debug)" -ldflags '$(ldflags-debug)'
# check for nostrip option
ifeq (,$(findstring nostrip,$(QUASAR_BUILD_OPTIONS)))
  BUILD_FLAGS += -trimpath
endif

###############################################################################
###                            Build & Install                              ###
###############################################################################

update-deps:
	@go mod tidy;

build: build-check-version go.sum
	# back up before build
	@cp go.mod go.mod.backup
	@cp go.sum go.sum.backup
	@go mod tidy

	mkdir -p $(BUILDDIR)/
	GOWORK=off go build -mod=readonly $(BUILD_FLAGS) -o $(BUILDDIR)/ $(GO_MODULE)/cmd/quasard

	# clean up before install
	@mv go.mod.backup go.mod
	@mv go.sum.backup go.sum
	@rm -f go.mod.bak
	@go mod tidy

install: build-check-version go.sum
	# back up before build
	@cp go.mod go.mod.backup
	@cp go.sum go.sum.backup
	@go mod tidy

	GOWORK=off go install -mod=readonly $(BUILD_FLAGS) $(GO_MODULE)/cmd/quasard

	# clean up before install
	@mv go.mod.backup go.mod
	@mv go.sum.backup go.sum
	@rm -f go.mod.bak
	@go mod tidy

###############################################################################
###                                Go Mock                                  ###
###############################################################################

# todo : need ideas on external libraries
# example : mockgen -source=/path/to/go/pkg/mod/github.com/cosmos/ibc-go/v7@v7.4.0/modules/core/05-port/types/module.go -destination=/path/to/quasar/mock/ics4_wrapper_mocks.go -package=mock -mock_names=MockICS4Wrapper
mocks: $(MOCKSDIR)/
	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_channel_mocks.go $(GOMOD)/x/qoracle/types ChannelKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ica_mocks.go $(GOMOD)/x/intergamm/types ICAControllerKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_mocks.go $(GOMOD)/x/intergamm/types IBCTransferKeeper
	mockgen -package=mock -destination=$(MOCKSDIR)/ics4_wrapper_mocks.go $(GOMOD)/x/qoracle/types ICS4Wrapper
	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_port_mocks.go $(GOMOD)/x/qoracle/types PortKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_connection_mocks.go $(GOMOD)/x/intergamm/types ConnectionKeeper
#	mockgen -package=mock -destination=$(MOCKSDIR)/ibc_client_mocks.go $(GOMOD)/x/intergamm/types ClientKeeper

$(MOCKSDIR)/:
	mkdir -p $(MOCKSDIR)/


.PHONY: all build-linux install format lint build \
	test test-all test-build test-cover test-unit test-race benchmark
