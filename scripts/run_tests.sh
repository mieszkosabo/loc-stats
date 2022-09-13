#!/bin/bash

set -x
set -eo pipefail

# we run the tests sequentialy because get_stats changes cwd internally
cargo test -- --test-threads 1
rm -rf tmp

echo "OK"