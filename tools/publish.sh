#!/bin/bash

set -e

PACKAGES="
  nuts-bytes-derive
  nuts-bytes
  nuts-backend
  nuts-tool-api
  nuts-memory
  nuts-directory
  nuts-container
  nuts-archive
  nuts-tool
"

for p in $PACKAGES; do
  cargo publish -p $p $@
done
