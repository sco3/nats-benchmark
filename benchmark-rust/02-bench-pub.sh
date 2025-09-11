#!/usr/bin/env -S bash

set -xueo pipefail
# source 01-create-stream.sh

source 00-server.sh

source 03-bench-sub.sh

target/release/pubone  --stream asdf --subject asdf.1 --msg-size 32000 --num-messages $MESSAGES &> bench-pub.log
#perf record --call-graph dwarf -F 99 ./target/release/nats-benchmark --stream asdf --subject asdf.1 --msg-size 32000 --num-messages $MESSAGES --concurrency 500

