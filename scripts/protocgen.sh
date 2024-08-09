#!/usr/bin/env bash

set -o xtrace
#set -Cue -o pipefail

project_dir="$(cd "$(dirname "${0}")/.." ; pwd)" # Absolute path to project dir
echo "project dir - $project_dir"
trap "rm -rf github.com" 0

proto_dirs=$(find ${project_dir}/proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)

echo "DIRES - $proto_dirs"

for dir in $proto_dirs; do
  echo "DIR - $dir"
  for file in $(find "${dir}" -maxdepth 1 -name '*.proto'); do
    echo "FILE - $file"
    if grep go_package $file &>/dev/null; then
      echo "Before buf"
      PWD=$(pwd)
      echo "PWD is - $PWD"
      buf generate --template "${project_dir}/proto/buf.gen.gogo.yaml" $file
    fi
  done
done

# Remove old protobuf generated go files
# find ${project_dir} -path "github.com" -prune -and -name "*.pb*.go" -type f -delete
echo "Copying ..."
PWD=$(pwd)
echo "PWD is - $PWD"
# Copy the generated go files over
cp -r github.com/quasar-finance/quasar/* .
cp -r github.com github.com.bkp
rm -r github.com.bkp



