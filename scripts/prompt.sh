#!/bin/sh
# Send one JSONL prompt to ruptured and print replies.
# Usage: scripts/prompt.sh "open chromium https://example.com"

set -eu
TEXT=${1:-}
if [ -z "$TEXT" ]; then
  echo "usage: $0 <prompt text>" >&2
  exit 1
fi

SOCK=${RUPTURE_SOCK:-}
if [ -z "$SOCK" ]; then
  if [ -S /run/rupture/rupture.sock ]; then
    SOCK=/run/rupture/rupture.sock
  else
    SOCK=${HOME}/.rupture/rupture.sock
  fi
fi

ID=$(date +%s)
LINE=$(printf '%s' "$TEXT" | python3 -c 'import json,sys; print(json.dumps({"type":"prompt","id":sys.argv[1],"text":sys.stdin.read()}))' "$ID")

printf '%s\n' "$LINE" | socat - UNIX-CONNECT:"$SOCK"
