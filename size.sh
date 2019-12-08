#!/bin/bash

set -euxo pipefail

cargo +nightly xbuild --release

arm-none-eabi-size target/thumbv7em-none-eabihf/release/tiny-nrf52 > size.txt

echo >> size.txt
echo "===============" >> size.txt
echo >> size.txt

# cargo bloat --release -n 1000 >> size.txt
