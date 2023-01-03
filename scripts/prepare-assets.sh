#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

TARGET_DIR=digs-"$RELEASE_VERSION"/

mkdir "$TARGET_DIR"

bin="digs"
if [ "$OS" = "windows-2022" ]; then
  bin="digs.exe"
fi
cp "target/$TARGET/release/$bin" "$TARGET_DIR"
