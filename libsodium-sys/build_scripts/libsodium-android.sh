#!/bin/sh

echo "Cd into $1"
cd $1
ARCHIVE_NAME=libsodium.tar.gz
DIR_NAME=libsodium-stable
rm -rf $DIR_NAME
curl https://download.libsodium.org/libsodium/releases/LATEST.tar.gz  -o $ARCHIVE_NAME
tar -xf $ARCHIVE_NAME

cd $DIR_NAME

export ANDROID_NDK_HOME=$NDK_HOME
sh dist-build/android-arm.sh

