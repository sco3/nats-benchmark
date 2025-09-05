#!/usr/bin/env -S bash


source 00-server.sh

nats stream add asdf --subjects asdf.1  --retention=interest --max-age="5m" --defaults $SERVER