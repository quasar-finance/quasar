#!/usr/bin/env bash

set -Cue -o pipefail

project_dir="$(cd "$(dirname "${0}")/.." ; pwd)" # Absolute path to project dir
docs_dir="${project_dir}/docs"
tmp_dir="${project_dir}/tmp-swagger-gen"

# Get the path of the cosmos-sdk repo from go/pkg/mod
locate_cosmos_sdk_dir() {
  go list -f "{{ .Dir }}" -m github.com/cosmos/cosmos-sdk
}

# Get the path of the ibc-go repo from go/pkg/mod
locate_ibc_go_dir() {
  go list -f "{{ .Dir }}" -m github.com/cosmos/ibc-go/v3
}

# Collect all proto dirs
collect_proto_dirs() {
  find "$@" -path -prune -o -name "*.proto" -print0 | xargs -0 -n1 dirname | sort | uniq
}

mkdir -p "$tmp_dir"
trap "rm -rf ${tmp_dir}" 0

cosmos_sdk_dir="$(locate_cosmos_sdk_dir)"
ibc_go_dir="$(locate_ibc_go_dir)"

while read -r proto_dir <&3 ; do
  query_file="$(find "${proto_dir}" -maxdepth 1 \( -name 'query.proto' -o -name 'service.proto' \))"
  if [[ ! -z "$query_file" ]]; then
    echo "$query_file"
    protoc \
      -I "${project_dir}/proto" \
      -I "${cosmos_sdk_dir}/third_party/proto" \
      -I "${cosmos_sdk_dir}/proto" \
      -I "${ibc_go_dir}/proto" \
      "$query_file" \
      --swagger_out "$tmp_dir" \
      --swagger_opt logtostderr=true \
      --swagger_opt fqn_for_swagger_name=true \
      --swagger_opt simple_operation_ids=true
  fi
done 3< <(collect_proto_dirs "${project_dir}/proto" "${cosmos_sdk_dir}/proto")

(
  cd "$docs_dir"

  yarn install
  yarn combine
  yarn convert
)
