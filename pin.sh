#!/bin/bash

pin() {
	local path exit
	path=$( $HOME/.pin/_pin "$@" )
	exit="$?"

	case "$exit" in
		2) cd "$path" ;;
		1) echo "$path" ;;
	esac
}

