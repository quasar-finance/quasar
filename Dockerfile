# syntax=docker/dockerfile:1

ARG GO_VERSION="1.19"
ARG RUNNER_IMAGE="ubuntu"

# --------------------------------------------------------
# Builder
# --------------------------------------------------------

FROM golang:${GO_VERSION}-ubuntu as builder


# Download go dependencies
WORKDIR /quasar
COPY go.mod go.sum ./
RUN --mount=type=cache,target=/root/.cache/go-build \
    --mount=type=cache,target=/root/go/pkg/mod \
    go mod download



# Copy the remaining files
COPY . .

# Build quasarnoded binary
# force it to use static lib (from above) not standard libgo_cosmwasm.so file
# then log output of file /quasar/build/quasarnoded
# then ensure static linking
RUN go build -o build/quasarnoded ./cmd/quasarnoded

# --------------------------------------------------------
# Runner
# --------------------------------------------------------

FROM ${RUNNER_IMAGE}

COPY --from=builder /quasar/build/quasarnoded /bin/quasarnoded

ENV HOME /quasar
WORKDIR $HOME

EXPOSE 26656
EXPOSE 26657
EXPOSE 1317

ENTRYPOINT ["quasarnoded"]