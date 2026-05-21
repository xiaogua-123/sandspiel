#!/bin/bash
# macOS 一键构建脚本 - 在 macOS 虚拟机中运行
set -e

echo "=== 1. 检查依赖 ==="
command -v node  >/dev/null 2>&1 || { echo "安装 Node.js: brew install node"; exit 1; }
command -v rustc >/dev/null 2>&1 || { echo "安装 Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"; exit 1; }

echo "=== 2. 安装 wasm-pack ==="
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

echo "=== 3. 安装 npm 依赖 ==="
npm install

echo "=== 4. 构建前端 (WASM + JS) ==="
npm run build

echo "=== 5. 构建 Tauri macOS 应用 ==="
cd src-tauri
cargo tauri build --target universal-apple-darwin 2>&1 || cargo tauri build 2>&1

echo ""
echo "=== 完成! ==="
echo "DMG 位置: src-tauri/target/release/bundle/dmg/"
ls -la src-tauri/target/release/bundle/dmg/ 2>/dev/null || \
ls -la src-tauri/target/universal-apple-darwin/release/bundle/dmg/ 2>/dev/null || \
echo "查找 DMG: find src-tauri/target -name '*.dmg'"
