#!/usr/bin/env sh

BINARY=/quasarnoded/${BINARY:-quasarnoded}
ID=${ID:-0}
LOG=${LOG:-quasarnoded.log}

if ! [ -f "${BINARY}" ]; then
	echo "The binary $(basename "${BINARY}") cannot be found. Please add the binary to the shared folder. Please use the BINARY environment variable if the name of the binary is not 'quasarnoded'"
	exit 1
fi

BINARY_CHECK="$(file "$BINARY" | grep 'ELF 64-bit LSB executable, x86-64')"

if [ -z "${BINARY_CHECK}" ]; then
	echo "Binary needs to be OS linux, ARCH amd64"
	exit 1
fi

export QUASARNODEDHOME="/quasarnoded/node${ID}/quasarnoded"

if [ -d "$(dirname "${QUASARNODEDHOME}"/"${LOG}")" ]; then
  "${BINARY}" --home "${QUASARNODEDHOME}" "$@" | tee "${QUASARNODEDHOME}/${LOG}"
else
  "${BINARY}" --home "${QUASARNODEDHOME}" "$@"
fi
