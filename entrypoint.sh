#!/bin/bash -eu

cargo uninstall systemfd cargo-watch

mv target/release/example /bin
cd / && rm -rf /usr/src/example

/bin/example
