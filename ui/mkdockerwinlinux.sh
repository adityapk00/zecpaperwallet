#!/bin/bash

# This is meant to be run inside the docker container for the compile environment


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

cd /opt/zecpaperwallet/ui
source ~/.cargo/env

# We need to run qmake before we run disclean
/opt/Qt/5.11.2/static/bin/qmake papersapling.pro CONFIG+=release
make distclean
rm -rf   artifacts/linux-zecpaperwallet-v$APP_VERSION
mkdir -p artifacts/linux-zecpaperwallet-v$APP_VERSION
/opt/Qt/5.11.2/static/bin/qmake papersapling.pro CONFIG+=release
make -j4

strip zecpaperwalletui
cp zecpaperwalletui artifacts/linux-zecpaperwallet-v$APP_VERSION

# Run qmake before distclean 
/opt/mxe/usr/bin/x86_64-w64-mingw32.static-qmake-qt5 papersapling.pro CONFIG+=release
make distclean
rm -rf   artifacts/Windows-zecpaperwallet-v$APP_VERSION
mkdir -p artifacts/Windows-zecpaperwallet-v$APP_VERSION
/opt/mxe/usr/bin/x86_64-w64-mingw32.static-qmake-qt5 papersapling.pro CONFIG+=release
make -j4

strip release/zecpaperwalletui.exe
cp release/zecpaperwalletui.exe artifacts/Windows-zecpaperwallet-v$APP_VERSION

# Cleanup before exiting
make distclean