clean:
	rm -rf build

rust: src/main.rs
	mkdir -p build
	rustc src/main.rs -o build/os
