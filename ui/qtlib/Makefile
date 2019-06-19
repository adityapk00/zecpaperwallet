ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := a
endif

all: release

release: target/release/zecpaperrust.$(EXT)
debug: target/debug/zecpaperrust.$(EXT) 

target/release/zecpaperrust.$(EXT): src/lib.rs Cargo.toml
	cargo build --lib --release

target/debug/zecpaperrust.$(EXT): src/lib.rs Cargo.toml
	cargo build --lib

clean:
	rm -rf target