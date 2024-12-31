rm executable/*

cargo build --release
cp target/release/flight_3 executable/flight_3_mac_arm

cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/flight_3 executable/flight_3_mac_x86_64

cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/flight_3.exe executable/flight_3_windows.exe

CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --release --target=x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/flight_3 executable/flight_3_linux_x86_64