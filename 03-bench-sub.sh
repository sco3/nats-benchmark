



set -xueo pipefail
source 00-server.sh


tmux new-session -d  -s  sub \
  "nats bench js fetch $SERVER --no-progress --stream asdf --size 32000 --filter \"asdf.1\" --msgs $MESSAGES 2>bench-sub.log.2 > bench-sub.log"
