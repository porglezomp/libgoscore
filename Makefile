example: target/example

target/release/libgoscore.a: src/*.rs
	cargo build --release

target/example: target/release/libgoscore.a main.c
	gcc -o target/example main.c -L target/release/ -l goscore
