#!/usr/bin/env bash

set -Cue -o pipefail

ignite chain serve -c config.yml --reset-once -v
