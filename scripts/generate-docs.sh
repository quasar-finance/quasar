#!/usr/bin/bash
echo "Starting to generate api docs"

set -Cue -o pipefail

project_dir="$(cd "$(dirname "${0}")/.." ; pwd)" # Absolute path to project dir
echo "Project dir : ${project_dir}"
docs_dir="${project_dir}/docs"
echo "docs_dir  : ${docs_dir}"

tmp_dir="${project_dir}/tmp-swagger-gen"
echo "tmp_dir : ${tmp_dir}"

mkdir -p $tmp_dir
trap "rm -rf ${tmp_dir}" 0

proto_dirs=$(find ${project_dir}/proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
echo "proto_dirs : ${proto_dirs}"

echo "================================================"
for dir in $proto_dirs; do
  # generate swagger files (filter query files)
  echo "inside loop - dir : ${dir}"

  query_file=$(find "${dir}" -maxdepth 1 \( -name 'query.proto' -o -name 'service.proto' \))
  echo "query file : ${query_file}"
  if [[ ! -z "$query_file" ]]; then
    echo "generate file for query file : ${query_file}"
    buf generate --template "${project_dir}/proto/buf.gen.swagger.yaml" $query_file
  fi
done

(
  cd "$docs_dir"
  # NOTE - You might need to run the below commands in your local
  # in case they are not properly installed. Yarn sometimes does not work
  # on some systems
  npm run combine
  npm run convert
  # yarn install
  # yarn combine
  # yarn convert
)
