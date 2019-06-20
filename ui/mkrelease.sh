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
make distclean >/dev/null 2>&1
rm -rf    artifacts/macOS-zecpaperwallet-v$APP_VERSION
mkdir -p  artifacts/macOS-zecpaperwallet-v$APP_VERSION
echo "[OK]"


echo -n "Configuring............"
# Build
$QT_STATIC/qmake papersapling.pro CONFIG+=release >/dev/null
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
docker run --rm -v ${PWD}/..:/opt/zecpaperwallet zecwallet/compileenv:v0.7 bash -c "cd /opt/zecpaperwallet/ui && ./mkdockerwinlinux.sh -v $APP_VERSION"

