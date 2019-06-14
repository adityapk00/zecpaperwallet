FROM rust:1.34
LABEL Description="Rust compile env for Linux + Windows (cross)"

RUN apt update
RUN apt install -y build-essential mingw-w64

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-unknown-linux-musl

# Append the linker to the cargo config for Windows cross compile
RUN echo "[target.x86_64-pc-windows-gnu]" >> /usr/local/cargo/config && \
    echo "linker = '/usr/bin/x86_64-w64-mingw32-gcc'" >> /usr/local/cargo/config

ENV CC_x86_64_unknown_linux_musl="gcc" 
