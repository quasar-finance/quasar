# syntax=docker/dockerfile:1

ARG GO_VERSION="1.20"
ARG RUNNER_IMAGE="gcr.io/distroless/static-debian11"

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
RUN ARCH=$(uname -m) && WASMVM_VERSION=$(go list -m github.com/CosmWasm/wasmvm | sed 's/.* //') && \
    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/libwasmvm_muslc.$ARCH.a \
        -O /lib/libwasmvm_muslc.a && \
    # verify checksum
    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/checksums.txt -O /tmp/checksums.txt && \
    sha256sum /lib/libwasmvm_muslc.a | grep $(cat /tmp/checksums.txt | grep libwasmvm_muslc.$ARCH | cut -d ' ' -f 1)

# Copy the remaining files
COPY . .

# Build quasarnoded binary
# force it to use static lib (from above) not standard libgo_cosmwasm.so file
# then log output of file /quasar/build/quasarnoded
# then ensure static linking
RUN --mount=type=cache,target=/root/.cache/go-build \
    --mount=type=cache,target=/root/go/pkg/mod \
    GOWORK=off go build \
            -mod=readonly \
            -tags "netgo,ledger,muslc" \
            -ldflags \
                "-X github.com/cosmos/cosmos-sdk/version.Name="quasar" \
                -X github.com/cosmos/cosmos-sdk/version.AppName="quasarnoded" \
                -X github.com/cosmos/cosmos-sdk/version.Version=${GIT_VERSION} \
                -X github.com/cosmos/cosmos-sdk/version.Commit=${GIT_COMMIT} \
                -X github.com/cosmos/cosmos-sdk/version.BuildTags='netgo,ledger,muslc' \
                -w -s -linkmode=external -extldflags '-Wl,-z,muldefs -static'" \
            -trimpath \
    -o build/quasarnoded \
    /quasar/cmd/quasarnoded/main.go


# --------------------------------------------------------
# Runner
# --------------------------------------------------------

FROM ${RUNNER_IMAGE} as runner

COPY --from=builder /quasar/build/quasarnoded /bin/quasarnoded

ENV HOME /quasar
WORKDIR $HOME

EXPOSE 26656
EXPOSE 26657
EXPOSE 1317

CMD ["quasarnoded"]

# --------------------------------------------------------
# Development
# --------------------------------------------------------

FROM ubuntu:22.04 as dev

ENV PACKAGES jq

RUN rm -f /etc/apt/apt.conf.d/docker-clean
RUN --mount=type=cache,target=/var/cache/apt \
	apt-get update && apt-get install -y $PACKAGES


COPY --from=builder /quasar/build/quasarnoded /bin/quasarnoded


ENV HOME /quasar
WORKDIR $HOME

COPY tests/docker/bootstrap-scripts/entrypoint.sh /quasar/entrypoint.sh
COPY tests/docker/bootstrap-scripts/quasar_localnet.sh /quasar/app_init.sh
RUN chmod +x entrypoint.sh && chmod +x app_init.sh && mkdir logs

EXPOSE 26656
EXPOSE 26657
EXPOSE 1317

CMD ["quasarnoded"]
ENTRYPOINT ["./entrypoint.sh"]
