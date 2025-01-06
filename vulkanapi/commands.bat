gcc -c -m64 -o src/printer.o src/printer.c
ar rcs lib/libprinter.a src/printer.o
cargo run