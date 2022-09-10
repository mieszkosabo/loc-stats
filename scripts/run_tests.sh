#!/bin/bash

set -x
set -eo pipefail

cargo test
rm -rf tmp

echo "OK"