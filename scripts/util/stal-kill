#!/usr/bin/env bash

PID={{PID}}
LOG={{LOG}}

while (($#)); do
	case "$1" in
		-l | --logs) OPEN_LOG=true ;;
	esac
	shift
done

kill $PID || echo "$PID not found"

[[ -n $OPEN_LOG ]] && (less $LOG)

rm -- ${BASH_SOURCE[0]}
