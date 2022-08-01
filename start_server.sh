#!/usr/bin/env bash
set -eu

# Starts a local web-server that serves the contents of the `doc/` folder,
# which is the folder to where the web version is compiled.

cargo install basic-http-server

echo "open http://localhost:42066"

(cd docs && basic-http-server --addr 127.0.0.1:42066 .)
