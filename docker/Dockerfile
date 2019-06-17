FROM rust:1.34
LABEL Description="Rust compile env for Linux + Windows (cross)"

RUN apt update
RUN apt install -y build-essential mingw-w64 gcc-aarch64-linux-gnu

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-gnu

# Append the linker to the cargo config for Windows cross compile
RUN echo "[target.x86_64-pc-windows-gnu]" >> /usr/local/cargo/config && \
    echo "linker = '/usr/bin/x86_64-w64-mingw32-gcc'" >> /usr/local/cargo/config

RUN echo "[target.aarch64-unknown-linux-gnu]" >> /usr/local/cargo/config && \
    echo "linker = '/usr/bin/aarch64-linux-gnu-gcc'" >> /usr/local/cargo/config

ENV CC_x86_64_unknown_linux_musl="gcc" 
ENV CC_aarch64_unknown_linux_gnu="aarch64-linux-gnu-gcc" 
