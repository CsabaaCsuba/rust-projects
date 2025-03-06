cargo rustc --release -- -C link-arg=--script=./linker.ld -C debuginfo=0 -C link-arg=-Wl,-Map=output.map
