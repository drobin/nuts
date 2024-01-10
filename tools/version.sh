#!/bin/bash

set -e

BASE_DIR="$(dirname $0)/.."
VERSION=$1

for f in $BASE_DIR/nuts-*/Cargo.toml; do
  sed -i '' -E "s/^version = \"(.*)\"/version = \"$VERSION\"/" $f
  sed -i '' -E "s/(nuts-[a-z-]+ = .*version = \"=)[^\"]+/\1$VERSION/g" $f
done
