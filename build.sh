#!/bin/sh
rm executable/*

cargo test -q
if [ 0 -ne $? ];
then
    exit 1
fi
echo "tests pass"

cargo build --release -q
cp target/release/flight_3 executable/flight_3_mac_arm
echo "built mac arm"

cargo build --release --target x86_64-apple-darwin -q
cp target/x86_64-apple-darwin/release/flight_3 executable/flight_3_mac_x86_64
echo "built mac intel"

cargo build --release --target x86_64-pc-windows-gnu -q
cp target/x86_64-pc-windows-gnu/release/flight_3.exe executable/flight_3_windows.exe
echo "built windows intel"

CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --release --target=x86_64-unknown-linux-gnu -q
cp target/x86_64-unknown-linux-gnu/release/flight_3 executable/flight_3_linux_x86_64
echo "built linux intel"