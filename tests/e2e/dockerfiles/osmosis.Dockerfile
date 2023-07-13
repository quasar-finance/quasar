# syntax=docker/dockerfile:1

ARG GO_VERSION="1.20.6"
ARG WASMVM_VERSION="v1.2.3"
ARG RUNNER_IMAGE="gcr.io/distroless/static-debian11"
# --------------------------------------------------------
# Builder
# --------------------------------------------------------

FROM golang:${GO_VERSION}-alpine as builder

ARG GIT_VERSION
ARG GIT_COMMIT
ARG WASMVM_VERSION

RUN apk add --no-cache \
    ca-certificates \
    build-base \
    linux-headers \
    git \
    git-lfs
RUN git lfs install

# Clone the osmosis repository
RUN git clone https://github.com/osmosis-labs/osmosis.git

# Checkout specific version
RUN cd osmosis && git checkout v16.1.0

# Set Work Directory to osmosis
WORKDIR osmosis

# Download go dependencies
RUN --mount=type=cache,target=/root/.cache/go-build \
    --mount=type=cache,target=/root/go/pkg/mod \
    go mod download

# Cosmwasm - Download correct libwasmvm version
RUN wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/libwasmvm_muslc.$(uname -m).a \
        -O /lib/libwasmvm_muslc.a
# verify checksum
#RUN wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/checksums.txt -O /tmp/checksums.txt && \
    #sha256sum /lib/libwasmvm_muslc.a | grep $(cat /tmp/checksums.txt | grep $(uname -m) | cut -d ' ' -f 1)

# Build osmosisd binary
RUN --mount=type=cache,target=/root/.cache/go-build \
    --mount=type=cache,target=/root/go/pkg/mod \
    GOWORK=off go build \
        -mod=readonly \
        -tags "netgo,ledger,muslc" \
        -ldflags \
            "-X github.com/cosmos/cosmos-sdk/version.Name="osmosis" \
            -X github.com/cosmos/cosmos-sdk/version.AppName="osmosisd" \
            -X github.com/cosmos/cosmos-sdk/version.Version=${GIT_VERSION} \
            -X github.com/cosmos/cosmos-sdk/version.Commit=${GIT_COMMIT} \
            -X github.com/cosmos/cosmos-sdk/version.BuildTags='netgo,ledger,muslc' \
            -w -s -linkmode=external -extldflags '-Wl,-z,muldefs -static'" \
        -trimpath \
        -o /osmosis/build/osmosisd \
        cmd/osmosisd/main.go

# --------------------------------------------------------
# Runner
# --------------------------------------------------------

FROM alpine:3.17.2

ENV PACKAGES bash

RUN apk add --no-cache $PACKAGES

COPY --from=builder /osmosis/build/osmosisd /bin/osmosisd

ENV HOME /osmosis
WORKDIR $HOME

EXPOSE 26656
EXPOSE 26657
EXPOSE 1317
