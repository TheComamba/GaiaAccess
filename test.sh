#!/bin/bash

set -e

cargo test --all-features -- --include-ignored
