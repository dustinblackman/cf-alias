#!/bin/bash

set -e

target="$1"

goTargetToRust() {
	if [[ "$target" == "darwin_amd64_v1" ]]; then
		echo "x86_64-apple-darwin"
	elif [[ "$target" == "darwin_arm64" ]]; then
		echo "aarch64-apple-darwin"
	elif [[ "$target" == "linux_amd64_v1" ]]; then
		echo "x86_64-unknown-linux-gnu"
	elif [[ "$target" == "linux_arm64" ]]; then
		echo "aarch64-unknown-linux-gnu"
	elif [[ "$target" == "windows_amd64_v1" ]]; then
		echo "x86_64-pc-windows-msvc"
	else
		echo "goreleaser-dist.sh is not prepared to handle builds for ${target}. Please update script."
		exit 1
	fi
}

rm -rf "./dist/cf-alias_${target}"
mkdir -p "./dist/cf-alias_${target}"

rustbin="./target/$(goTargetToRust)/release/cf-alias"
if [[ "$target" == "windows_amd64_v1" ]]; then
	rustbin="${rustbin}.exe"
fi

cp "$rustbin" "./dist/cf-alias_${target}/"
