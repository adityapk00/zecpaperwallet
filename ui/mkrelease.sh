#!/bin/bash

# Accept the variables as command line arguments as well
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


if [ -z $APP_VERSION ]; then
    echo "APP_VERSION is not set. Please set it to the current release version of the app";
    exit 1;
fi

# This should be set as an environment variable
if [ -z $QT_PATH ]; then 
    echo "QT_PATH is not set. Please set it to the base directory of Qt"; 
    exit 1; 
fi
QT_STATIC=$QT_PATH/clang_64/bin

# Build for MacOS first

# Clean
echo -n "Cleaning..............."
$QT_STATIC/qmake papersapling.pro CONFIG+=release >/dev/null
make distclean >/dev/null 2>&1
rm -rf    artifacts/macOS-zecpaperwallet-v$APP_VERSION
mkdir -p  artifacts/macOS-zecpaperwallet-v$APP_VERSION
echo "[OK]"


echo -n "Configuring............"
# Build
$QT_STATIC/qmake papersapling.pro CONFIG+=release >/dev/null
APP_BUILD_DATE=$(date +%F)
echo "#define APP_VERSION \"$APP_VERSION\"" > src/version.h
echo "#define APP_BUILD_DATE \"$APP_BUILD_DATE\"" >> src/version.h

echo "[OK]"


echo -n "Building..............."
make -j4 >/dev/null
echo "[OK]"

#Qt deploy
echo -n "Deploying.............."
$QT_STATIC/macdeployqt zecpaperwalletui.app 
cp -r zecpaperwalletui.app artifacts/macOS-zecpaperwallet-v$APP_VERSION/
echo "[OK]"

# Run inside docker container
docker run --rm -v ${PWD}/..:/opt/zecpaperwallet zecwallet/compileenv:v0.8 bash -c "cd /opt/zecpaperwallet/ui && ./mkdockerwinlinux.sh -v $APP_VERSION"


# Move to build the cli
cd ../cli

# Clean everything first
cargo clean
echo "pub fn version() -> &'static str { &\"$APP_VERSION\" }" > src/version.rs

# Compile for mac directly
cargo build --release 

# For Windows and Linux, build via docker
docker run --rm -v $(pwd)/..:/opt/zecpaperwallet rust/zecpaperwallet:v0.2 bash -c "cd /opt/zecpaperwallet/cli && cargo build --release --target x86_64-unknown-linux-musl && cargo build --release --target x86_64-pc-windows-gnu && cargo build --release --target aarch64-unknown-linux-gnu"

# Come back and package everything
cd ../ui

# Now sign and zip the binaries
#macOS
cp ../cli/target/release/zecpaperwallet artifacts/macOS-zecpaperwallet-v$APP_VERSION/
gpg --batch --output artifacts/macOS-zecpaperwallet-v$APP_VERSION/zecpaperwallet.sig --detach-sig artifacts/macOS-zecpaperwallet-v$APP_VERSION/zecpaperwallet 
#gpg --batch --output artifacts/macOS-zecpaperwallet-v$APP_VERSION/zecpaperwallet.app.sig --detach-sig artifacts/macOS-zecpaperwallet-v$APP_VERSION/zecpaperwallet.app 
cd artifacts
cd macOS-zecpaperwallet-v$APP_VERSION
gsha256sum zecpaperwallet > sha256sum.txt
cd ..
zip -r macOS-zecpaperwallet-v$APP_VERSION.zip macOS-zecpaperwallet-v$APP_VERSION 
cd ..


#Linux
cp ../cli/target/x86_64-unknown-linux-musl/release/zecpaperwallet artifacts/linux-zecpaperwallet-v$APP_VERSION/
gpg --batch --output artifacts/linux-zecpaperwallet-v$APP_VERSION/zecpaperwallet.sig --detach-sig artifacts/linux-zecpaperwallet-v$APP_VERSION/zecpaperwallet
gpg --batch --output artifacts/linux-zecpaperwallet-v$APP_VERSION/zecpaperwalletui.sig --detach-sig artifacts/linux-zecpaperwallet-v$APP_VERSION/zecpaperwalletui
cd artifacts
cd linux-zecpaperwallet-v$APP_VERSION
gsha256sum zecpaperwallet zecpaperwalletui > sha256sum.txt
cd ..
zip -r linux-zecpaperwallet-v$APP_VERSION.zip linux-zecpaperwallet-v$APP_VERSION 
cd ..


#Windows
cp ../cli/target/x86_64-pc-windows-gnu/release/zecpaperwallet.exe artifacts/Windows-zecpaperwallet-v$APP_VERSION/
gpg --batch --output artifacts/Windows-zecpaperwallet-v$APP_VERSION/zecpaperwallet.sig --detach-sig artifacts/Windows-zecpaperwallet-v$APP_VERSION/zecpaperwallet.exe
gpg --batch --output artifacts/Windows-zecpaperwallet-v$APP_VERSION/zecpaperwalletui.sig --detach-sig artifacts/Windows-zecpaperwallet-v$APP_VERSION/zecpaperwalletui.exe
cd artifacts
cd Windows-zecpaperwallet-v$APP_VERSION
gsha256sum zecpaperwallet.exe zecpaperwalletui.exe > sha256sum.txt
cd ..
zip -r Windows-zecpaperwallet-v$APP_VERSION.zip Windows-zecpaperwallet-v$APP_VERSION 
cd ..


# aarch64 (armv8)
rm -rf artifacts/aarch64-zecpaperwallet-v$APP_VERSION
mkdir -p artifacts/aarch64-zecpaperwallet-v$APP_VERSION
cp ../cli/target/aarch64-unknown-linux-gnu/release/zecpaperwallet artifacts/aarch64-zecpaperwallet-v$APP_VERSION/
gpg --batch --output artifacts/aarch64-zecpaperwallet-v$APP_VERSION/zecpaperwallet.sig --detach-sig artifacts/aarch64-zecpaperwallet-v$APP_VERSION/zecpaperwallet
cd artifacts
cd aarch64-zecpaperwallet-v$APP_VERSION
gsha256sum zecpaperwallet > sha256sum.txt
cd ..
zip -r aarch64-zecpaperwallet-v$APP_VERSION.zip aarch64-zecpaperwallet-v$APP_VERSION 
cd ..

