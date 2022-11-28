#!/usr/bin/env bash

set -Cue -o pipefail

project_dir="$(cd "$(dirname "${0}")/.." ; pwd)" # Absolute path to project dir
docs_dir="${project_dir}/docs"
tmp_dir="${project_dir}/tmp-swagger-gen"

mkdir -p $tmp_dir
trap "rm -rf ${tmp_dir}" 0

proto_dirs=$(find ${project_dir}/proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
for dir in $proto_dirs; do
  # generate swagger files (filter query files)
  query_file=$(find "${dir}" -maxdepth 1 \( -name 'query.proto' -o -name 'service.proto' \))
  if [[ ! -z "$query_file" ]]; then
    buf generate --template "${project_dir}/proto/buf.gen.swagger.yaml" $query_file
  fi
done

(
  cd "$docs_dir"

  yarn install
  yarn combine
  yarn convert
)
