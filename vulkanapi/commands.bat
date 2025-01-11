::requires gcc64 to be installed

rustup toolchain install stable-gnu
rustup set default-host x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-gnu
rustup default stable-gnu
cargo run