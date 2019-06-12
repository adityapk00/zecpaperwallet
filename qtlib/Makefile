ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := a
endif

all: target/debug/zecpaperrust.$(EXT)
	g++ src/main.cpp -L./target/debug -lzecpaperrust -lpthread -ldl -o run
	./run

target/debug/zecpaperrust.$(EXT): src/lib.rs Cargo.toml
	cargo build --lib

clean:
	rm -rf target
	rm -rf run