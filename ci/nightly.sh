#!/bin/bash

set -o errexit -o nounset

cargo test
cargo test --features std
