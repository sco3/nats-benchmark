#!/usr/bin/env -S bash

set -xueo pipefail
# source 01-create-stream.sh

source 00-server.sh

source 03-bench-sub.sh


#uv run pub-batch.py -b 500  -s asdf --msg-size 32000 -sub asdf.1 -m $MESSAGES 2>bench-pub.log.2 | tee bench-pub.log
uv run pube.py   -s conversation-stream --msg-size 32000 -sub asdf.1 --num-messages $MESSAGES 2>bench-pub.log.2 > bench-pub.log