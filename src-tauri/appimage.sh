#!/bin/bash

# 设置工作目录
WORKDIR=$(pwd)

# 设置输出目录
OUTPUT_DIR="$WORKDIR/target/release/bundle/appimage"

# 确保输出目录存在
mkdir -p $OUTPUT_DIR

# 构建 AppImage
appimagetool-x86_64.AppImage $WORKDIR/target/release/bundle/appimage $OUTPUT_DIR/极速全文搜索_0.1.0_amd64.AppImage
