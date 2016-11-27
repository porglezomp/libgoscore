example: 

../target/release/libgoscore.a: ../src/*.rs
	cargo build --release

example: ../target/release/libgoscore.a example.c
	gcc -o example example.c -L ../target/release/ -l goscore -I ../src/
