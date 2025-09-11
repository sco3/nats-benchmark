#!/usr/bin/env -S bash

set -xueo pipefail
# source 01-create-stream.sh

source 00-server.sh

source 03-bench-sub.sh


bun pub.js -s asdf -j asdf.1 -m 32000 -n $MESSAGES