build:
	cargo build --release
	rm -f ./lua/libmy_module.so
	cp ./target/release/liblua_with_rust.dylib ./lib/liblua_with_rust.so
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
