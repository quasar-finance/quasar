#!/usr/bin/env bash

set -Cue -o pipefail

starport chain serve -c config.yml --reset-once -v
