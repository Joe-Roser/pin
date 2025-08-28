#!/bin/bash

pin() {
	local path exit
	path=$(/mnt/HDD8TB/code/rust/_pin/target/debug/_pin "$@")
	exit="$?"

	case "$exit" in
		"2") cd "$path" ;;
		"1") echo "$path" ;;
	esac
}

