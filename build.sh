#!/usr/bin/env bash
THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"

INDEX_FILE_PATH="${THIS_DIR}/src/index.ts"
DIST_DIR="${THIS_DIR}/dist"

mkdir -p "${DIST_DIR}"

function build() {
  local TARGET="$1"
  deno compile \
       --lite \
       --unstable \
       --allow-net \
       --allow-env \
       --target "${TARGET}" \
       --output "${DIST_DIR}/update-issue_${TARGET}" \
       "${INDEX_FILE_PATH}"
}

build "x86_64-unknown-linux-gnu"
build "x86_64-pc-windows-msvc"
build "x86_64-apple-darwin"
build "aarch64-apple-darwin"
