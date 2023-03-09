# syntax=docker/dockerfile:1

ARG GO_VERSION="1.19"
# todo remove unencessary flags.
ARG RUNNER_IMAGE="ubuntu"

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
RUN WASMVM_VERSION=$(go list -m github.com/CosmWasm/wasmvm | cut -d ' ' -f 2) && \
    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/libwasmvm_muslc.$(uname -m).a \
      -O /lib/libwasmvm_muslc.a && \
    # verify checksum
    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/checksums.txt -O /tmp/checksums.txt && \
    sha256sum /lib/libwasmvm_muslc.a | grep $(cat /tmp/checksums.txt | grep $(uname -m) | cut -d ' ' -f 1)

# Copy the remaining files
COPY . .

# Build quasarnoded binary
# force it to use static lib (from above) not standard libgo_cosmwasm.so file
# then log output of file /quasar/build/quasarnoded
# then ensure static linking
RUN --mount=type=cache,target=/root/.cache/go-build \
    --mount=type=cache,target=/root/go/pkg/mod \
    LEDGER_ENABLED=false BUILD_TAGS=muslc LINK_STATICALLY=true make build

# --------------------------------------------------------
# Runner
# --------------------------------------------------------

FROM alpine:3.17.2

ENV PACKAGES bash

RUN apk add --no-cache $PACKAGES

COPY --from=builder /quasar/build/quasarnoded /bin/quasarnoded

ENV HOME /quasar
WORKDIR $HOME

EXPOSE 26656
EXPOSE 26657
EXPOSE 1317

#ENTRYPOINT ["quasarnoded"]