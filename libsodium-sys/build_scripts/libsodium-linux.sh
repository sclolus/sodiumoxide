#!/bin/sh
ARCHIVE_NAME=libsodium.tar.gz
DIR_NAME=libsodium-stable

echo "Cd into $1"
cd $1
rm -rf $DIR_NAME
curl https://download.libsodium.org/libsodium/releases/LATEST.tar.gz -o $ARCHIVE_NAME
tar -xf $ARCHIVE_NAME
rm -f $ARCHIVE_NAME

cd $DIR_NAME
chmod +x ./configure

./configure --prefix=$(pwd)/libsodium-linux && make && make install
