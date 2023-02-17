#!/usr/bin/env bash
# This script does not work with cosmos-sdk v0.46 and newer verisons and therefore is DEPRECATED.
# for replacement please use protocgen.sh in this directroy.
set -Cue -o pipefail

project_dir="$(cd "$(dirname "${0}")/.." ; pwd)" # Absolute path to project dir
build_dir="${BUILD_DIR:-"${project_dir}/build"}"
tmp_dir="${build_dir}/proto"

# Get the path of the cosmos-sdk repo from go/pkg/mod
locate_cosmos_sdk_dir() {
  go list -f "{{ .Dir }}" -m github.com/cosmos/cosmos-sdk
}

# Get the path of the ibc-go repo from go/pkg/mod
locate_ibc_go_dir() {
  go list -f "{{ .Dir }}" -m github.com/cosmos/ibc-go/v4
}

# Collect all proto dirs
collect_proto_dirs() {
  find "$@" -path -prune -o -name "*.proto" -print0 | xargs -0 -n1 dirname | sort | uniq
}

mkdir -p "$tmp_dir"
trap "rm -rf ${tmp_dir}" 0

cosmos_sdk_dir="$(locate_cosmos_sdk_dir)"
ibc_go_dir="$(locate_ibc_go_dir)"

(
  cd "$project_dir"

  while read -r proto_child_dir <&3 ; do
  echo "$proto_child_dir"
    protoc \
      -I "${project_dir}/proto" \
      -I "${cosmos_sdk_dir}/third_party/proto" \
      -I "${cosmos_sdk_dir}/proto" \
      -I "${ibc_go_dir}/proto" \
      --gocosmos_out=plugins=interfacetype+grpc,\
Mgoogle/protobuf/any.proto=github.com/cosmos/cosmos-sdk/codec/types:"$tmp_dir" \
      --grpc-gateway_out=logtostderr=true:"$tmp_dir" \
      $(find "${proto_child_dir}" -name '*.proto')
  done 3< <(collect_proto_dirs "${project_dir}/proto")

  # Remove any protobuf generated go file
  find ${project_dir} -name "*.pb*.go" -not -path "${tmp_dir}/*" -type f -delete

  # Copy the generated go files over
  cp -r "${tmp_dir}/github.com/quasarlabs/quasarnode/"* .
)
