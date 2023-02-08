#!/usr/bin/env bash

set -Cue -o pipefail

project_dir="$(
  cd "$(dirname "${0}")/.."
  pwd
)" # Absolute path to project dir

trap "rm -rf github.com" 0

proto_dirs=$(find ${project_dir}/proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
for dir in $proto_dirs; do
  for file in $(find "${dir}" -maxdepth 1 -name '*.proto'); do
    if grep go_package $file &>/dev/null; then
      buf generate --template "${project_dir}/proto/buf.gen.gogo.yaml" $file
    fi
  done
done

# Copy the generated go files over
cp -r github.com/quasarlabs/quasarnode/* .
