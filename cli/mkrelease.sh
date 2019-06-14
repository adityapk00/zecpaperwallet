#!/bin/bash
# This script depends on a docker image already being built
# To build it, 
# cd docker
# docker build --tag rust/zecpaperwallet:v0.1 .

POSITIONAL=()
while [[ $# -gt 0 ]]
do
key="$1"

case $key in
    -v|--version)
    APP_VERSION="$2"
    shift # past argument
    shift # past value
    ;;
    *)    # unknown option
    POSITIONAL+=("$1") # save it in an array for later
    shift # past argument
    ;;
esac
done
set -- "${POSITIONAL[@]}" # restore positional parameters

if [ -z $APP_VERSION ]; then echo "APP_VERSION is not set"; exit 1; fi

# Clean everything first
cargo clean

# Compile for mac directly
cargo build --release 

# For Windows and Linux, build via docker
docker run --rm -v $(pwd)/..:/opt/zecpaperwallet rust/zecpaperwallet:v0.1 bash -c "cd /opt/zecpaperwallet/cli && cargo build --release --target x86_64-unknown-linux-musl && cargo build --release --target x86_64-pc-windows-gnu"

# Now sign and zip the binaries
#macOS
rm -rf target/macOS-zecpaperwallet-v$APP_VERSION
mkdir -p target/macOS-zecpaperwallet-v$APP_VERSION
cp target/release/zecpaperwallet target/macOS-zecpaperwallet-v$APP_VERSION/
gpg --batch --output target/macOS-zecpaperwallet-v$APP_VERSION/zecpaperwallet.sig --detach-sig target/macOS-zecpaperwallet-v$APP_VERSION/zecpaperwallet 
cd target
cd macOS-zecpaperwallet-v$APP_VERSION
gsha256sum zecpaperwallet > sha256sum.txt
cd ..
zip -r macOS-zecpaperwallet-v$APP_VERSION.zip macOS-zecpaperwallet-v$APP_VERSION 
cd ..


#Linux
rm -rf target/linux-zecpaperwallet-v$APP_VERSION
mkdir -p target/linux-zecpaperwallet-v$APP_VERSION
cp target/x86_64-unknown-linux-musl/release/zecpaperwallet target/linux-zecpaperwallet-v$APP_VERSION/
gpg --batch --output target/linux-zecpaperwallet-v$APP_VERSION/zecpaperwallet.sig --detach-sig target/linux-zecpaperwallet-v$APP_VERSION/zecpaperwallet
cd target
cd linux-zecpaperwallet-v$APP_VERSION
gsha256sum zecpaperwallet > sha256sum.txt
cd ..
zip -r linux-zecpaperwallet-v$APP_VERSION.zip linux-zecpaperwallet-v$APP_VERSION 
cd ..


#Windows
rm -rf target/Windows-zecpaperwallet-v$APP_VERSION
mkdir -p target/Windows-zecpaperwallet-v$APP_VERSION
cp target/x86_64-pc-windows-gnu/release/zecpaperwallet.exe target/Windows-zecpaperwallet-v$APP_VERSION/
gpg --batch --output target/Windows-zecpaperwallet-v$APP_VERSION/zecpaperwallet.sig --detach-sig target/Windows-zecpaperwallet-v$APP_VERSION/zecpaperwallet.exe
cd target
cd Windows-zecpaperwallet-v$APP_VERSION
gsha256sum zecpaperwallet.exe > sha256sum.txt
cd ..
zip -r Windows-zecpaperwallet-v$APP_VERSION.zip Windows-zecpaperwallet-v$APP_VERSION 
cd ..
