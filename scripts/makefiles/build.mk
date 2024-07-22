###############################################################################
###                                  Build                                  ###
###############################################################################

build-help:
	@echo "build subcommands"
	@echo ""
	@echo "Usage:"
	@echo "  make build-[command]"
	@echo ""
	@echo "Available Commands:"
	@echo "  all                              Build all targets"
	@echo "  check-version                    Check Go version"
	@echo "  dev-build                        Build development version"
	@echo "  dev-install                      Install development build"
	@echo "  linux                            Build for Linux"
	@echo "  reproducible                     Build reproducible binaries"
	@echo "  reproducible-amd64               Build reproducible amd64 binary"
	@echo "  reproducible-arm64               Build reproducible arm64 binary"

build-check-version:
	@echo "Go version: $(GO_MAJOR_VERSION).$(GO_MINOR_VERSION)"
	@if [ $(GO_MAJOR_VERSION) -gt $(GO_MINIMUM_MAJOR_VERSION) ]; then \
		echo "Go version is sufficient"; \
		exit 0; \
	elif [ $(GO_MAJOR_VERSION) -lt $(GO_MINIMUM_MAJOR_VERSION) ]; then \
		echo '$(GO_VERSION_ERR_MSG)'; \
		exit 1; \
	elif [ $(GO_MINOR_VERSION) -lt $(GO_MINIMUM_MINOR_VERSION) ]; then \
		echo '$(GO_VERSION_ERR_MSG)'; \
		exit 1; \
	fi

build-all: build-check-version go.sum
	mkdir -p $(BUILDDIR)/
	GOWORK=off go build -mod=readonly $(BUILD_FLAGS) -o $(BUILDDIR)/ ./...

build-linux: go.sum
	LEDGER_ENABLED=false GOOS=linux GOARCH=amd64 $(MAKE) build

# disables optimization, inlining and symbol removal
GC_FLAGS := -gcflags="all=-N -l"
REMOVE_STRING := -w -s
DEBUG_BUILD_FLAGS:= $(subst $(REMOVE_STRING),,$(BUILD_FLAGS))
DEBUG_LDFLAGS = $(subst $(REMOVE_STRING),,$(ldflags))

build-dev-install: go.sum
	GOWORK=off go install $(DEBUG_BUILD_FLAGS) $(GC_FLAGS) $(GO_MODULE)/cmd/quasarnoded

build-dev-build:
	mkdir -p $(BUILDDIR)/
	GOWORK=off go build $(GC_FLAGS) -mod=readonly -ldflags '$(DEBUG_LDFLAGS)' -gcflags "all=-N -l" -trimpath -o $(BUILDDIR) ./...;

all: install lint test

BUILD_TARGETS := build install
#BUILD_TARGETS_DEBUG := build install
build: BUILD_ARGS=-o $(BUILDDIR)/

$(BUILD_TARGETS): go.sum $(BUILDDIR)/
	GOWORK=off go $@ -mod=readonly $(BUILD_FLAGS) $(BUILD_ARGS) ./cmd/quasarnoded

$(BUILD_TARGETS_DEBUG): go.sum $(BUILDDIR)/
	GOWORK=off go $@ -mod=readonly $(BUILD_FLAGS_DEBUG) -gcflags='all=-N -l' $(BUILD_ARGS) ./cmd/quasarnoded

$(BUILDDIR)/:
	mkdir -p $(BUILDDIR)/

###############################################################################
###                          Build reproducible                             ###
###############################################################################

build-reproducible: build-reproducible-amd64 build-reproducible-arm64

build-reproducible-amd64: go.sum
	mkdir -p $(BUILDDIR)
	$(DOCKER) buildx create --name quasarbuilder || true
	$(DOCKER) buildx use quasarbuilder
	$(DOCKER) buildx build \
		--build-arg GO_VERSION=$(GO_VERSION) \
        --build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_DISTROLESS) \
        --build-arg GIT_VERSION=$(VERSION) \
        --build-arg GIT_COMMIT=$(COMMIT) \
        --platform linux/amd64 \
        -t quasar:local-amd64 \
        --load \
        -f Dockerfile .
	$(DOCKER) rm -f quasarbinary || true
	$(DOCKER) create -ti --name quasarbinary quasar:local-amd64
	$(DOCKER) cp quasarbinary:/bin/quasarnoded $(BUILDDIR)/quasarnoded-linux-amd64
	$(DOCKER) rm -f quasarbinary

build-reproducible-arm64: go.sum
	mkdir -p $(BUILDDIR)
	$(DOCKER) buildx create --name quasarbuilder || true
	$(DOCKER) buildx use quasarbuilder
	$(DOCKER) buildx build \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_DISTROLESS) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		--platform linux/arm64 \
		-t quasar-arm64 \
		--load \
		-f Dockerfile .
	$(DOCKER) rm -f quasarbinary || true
	$(DOCKER) create -ti --name quasarbinary quasar-arm64
	$(DOCKER) cp quasarbinary:/bin/quasarnoded $(BUILDDIR)/quasarnoded-linux-arm64
	$(DOCKER) rm -f quasarbinary

go.sum: go.mod
	@echo "--> Ensure dependencies have not been modified"
	@go mod verify
