# syntax=docker/dockerfile:1

ARG GO_VERSION="1.20.6"
ARG RUNNER_IMAGE="gcr.io/distroless/static"

# --------------------------------------------------------
# Builder
# --------------------------------------------------------

FROM golang:${GO_VERSION}-alpine as builder

ARG GIT_VERSION
ARG GIT_COMMIT

RUN apk add --no-cache \
    ca-certificates \
    build-base \
    linux-headers

# Download go dependencies
WORKDIR /quasar
COPY go.mod go.sum ./
RUN --mount=type=cache,target=/root/.cache/go-build \
    --mount=type=cache,target=/root/go/pkg/mod \
    go mod download

# Cosmwasm - Download correct libwasmvm version
RUN export WASMVM_VERSION=$(go list -m github.com/CosmWasm/wasmvm | awk '{print $NF}') && \
    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/libwasmvm_muslc.$(uname -m).a \
      -O /lib/libwasmvm_muslc.a && \
    # verify checksum
    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/checksums.txt -O /tmp/checksums.txt && \
    sha256sum /lib/libwasmvm_muslc.a | grep $(cat /tmp/checksums.txt | grep $(uname -m) | cut -d ' ' -f 1)

# Copy the remaining files
COPY . .

# Build quasard binary
# force it to use static lib (from above) not standard libgo_cosmwasm.so file
# then log output of file /quasar/build/quasard
# then ensure static linking
RUN --mount=type=cache,target=/root/.cache/go-build \
    --mount=type=cache,target=/root/go/pkg/mod \
    GOWORK=off go build \
            -mod=readonly \
            -tags "netgo,ledger,muslc" \
            -ldflags \
                "-X github.com/cosmos/cosmos-sdk/version.Name="quasar" \
                -X github.com/cosmos/cosmos-sdk/version.AppName="quasard" \
                -X github.com/cosmos/cosmos-sdk/version.Version=${GIT_VERSION} \
                -X github.com/cosmos/cosmos-sdk/version.Commit=${GIT_COMMIT} \
                -X github.com/cosmos/cosmos-sdk/version.BuildTags='netgo,ledger,muslc' \
                -w -s -linkmode=external -extldflags '-Wl,-z,muldefs -static'" \
            -trimpath \
    -o build/quasard ./cmd/quasard


# --------------------------------------------------------
# Runner
# --------------------------------------------------------

FROM alpine:3.17.2

ENV PACKAGES bash

RUN apk add --no-cache $PACKAGES

COPY --from=builder /quasar/build/quasard /bin/quasard

ENV HOME /quasar
WORKDIR $HOME

EXPOSE 26656
EXPOSE 26657
EXPOSE 1317
