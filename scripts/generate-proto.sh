#!/usr/bin/env bash

set -o xtrace
#set -Cue -o pipefail

# Determine the absolute path to the project directory
project_dir="$(cd "$(dirname "${0}")/.." ; pwd)"
echo "project dir - $project_dir"

# Ensure the temporary files are cleaned up
trap "rm -rf github.com" 0

# Find all directories containing .proto files
proto_dirs=$(find ${project_dir}/proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)

echo "DIRES - $proto_dirs"

# Loop through each directory containing .proto files
for dir in $proto_dirs; do
  echo "DIR - $dir"
  # Loop through each .proto file in the directory
  for file in $(find "${dir}" -maxdepth 1 -name '*.proto'); do
    echo "FILE - $file"
    # Check if the .proto file contains a go_package option
    if grep go_package $file &>/dev/null; then
      echo "Before buf"
      PWD=$(pwd)
      echo "PWD is - $PWD"
      # Generate Go code using buf
      buf generate --template "${project_dir}/proto/buf.gen.gogo.yaml" $file
    fi
  done
done

# Optional: Remove old protobuf generated go files
find ${project_dir} -path "github.com" -prune -and -name "*.pb*.go" -type f -delete

echo "Copying ..."
PWD=$(pwd)
echo "PWD is - $PWD"

# Copy the generated Go files to the desired location
cp -r github.com/quasar-finance/quasar/* .
cp -r github.com github.com.bkp