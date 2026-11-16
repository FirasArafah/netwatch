#!/usr/bin/env bash
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v cargo >/dev/null 2>&1; then
  echo "[ERROR] cargo not found. Install Rust from https://rustup.rs" >&2
  exit 1
fi

cmd="${1:-build}"

case "$cmd" in
  build)   cargo build ;;
  run)     cargo run ;;
  release) cargo build --release
           echo; echo "Binary: target/release/netwatch.exe" ;;
  test)    cargo test ;;
  check)   cargo check ;;
  clippy)  cargo clippy --all-targets -- -D warnings ;;
  fmt)     cargo fmt --all ;;
  dev)     cargo fmt --all
           cargo clippy --all-targets -- -D warnings
           cargo test
           cargo build ;;
  clean)   cargo clean ;;
  help|*)  cat <<'EOF'
Usage: ./build.sh [build|run|release|test|check|clippy|fmt|dev|clean|help]

  dev  = fmt -> clippy -> test -> build  (run before every commit)
EOF
  ;;
esac