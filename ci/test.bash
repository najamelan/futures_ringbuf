#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

# We can't run without all features for now, because that fails the
# doc tests of documentation on feature gated parts.
#
# cargo test --no-default-features
# cargo test
cargo test --all-features
