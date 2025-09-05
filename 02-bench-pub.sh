#!/usr/bin/env -S bash

set -xueo pipefail
# source 01-create-stream.sh

source 00-server.sh

source 03-bench-sub.sh

nats bench js pub $SERVER --batch 1  --no-progress --stream asdf --size 32000 asdf.1 --msgs $MESSAGES 2>bench-pub.log.2 | tee bench-pub.log