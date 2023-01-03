#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

if [ "$OS" = "windows-2022" ]; then
  7z a -tzip "digs-$RELEASE_VERSION-$TARGET.zip" digs-"$RELEASE_VERSION"/
else
  tar -czvf digs-"$RELEASE_VERSION"-"$TARGET".tar.gz digs-"$RELEASE_VERSION"/
  shasum -a 512 digs-"$RELEASE_VERSION"-"$TARGET".tar.gz >digs-"$RELEASE_VERSION"-"$TARGET".tar.gz.sha512
fi
