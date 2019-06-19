ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := a
endif

all: release

winrelease: target/x86_64-pc-windows-gnu/release/zecpaperrust.lib

target/x86_64-pc-windows-gnu/release/zecpaperrust.lib: src/lib.rs Cargo.toml
	cargo build --lib --release --target x86_64-pc-windows-gnu

release: target/release/zecpaperrust.$(EXT)
debug: target/debug/zecpaperrust.$(EXT) 

target/release/zecpaperrust.$(EXT): src/lib.rs Cargo.toml
	cargo build --lib --release

target/debug/zecpaperrust.$(EXT): src/lib.rs Cargo.toml
	cargo build --lib

clean:
	rm -rf target
