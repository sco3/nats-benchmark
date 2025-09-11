#!/usr/bin/env -S bash


source 00-server.sh

nats stream add conversation-stream --subjects asdf.1  --retention=interest --max-age="5m" --defaults $SERVER