#!/bin/bash
# Pre-commit quality checks for Rust projects

set -e

echo "🔍 Running pre-commit checks..."

echo ""
echo "📝 Formatting code..."
cargo fmt

echo ""
echo "🔎 Running clippy..."
cargo clippy --all-features -- -D warnings

echo ""
echo "🧪 Running tests..."
cargo test --all-features

echo ""
echo "🏗️  Building release..."
cargo build --release

echo ""
echo "✅ All checks passed! Ready to commit."
