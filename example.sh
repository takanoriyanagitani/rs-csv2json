#!/bin/sh

export MAX_LINES=255

mkdir -p sample.d

printf \
	'%s\n' \
	'name,height' \
	'fuji,3776' \
	'takao,599' \
	'sky,634' \
	'tokyo,333' |
	dd \
		if=/dev/stdin \
		of=./sample.d/input.csv \
		bs=1048576 \
		status=none

wazero \
	run \
	-mount "${PWD}/sample.d:/guest.d:ro" \
	rs-csv2json.wasm \
	"/guest.d/input.csv"
