{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer

    # Build dependencies
    pkg-config
    openssl

    # Database dependencies
    sqlite
    libspatialite
    postgresql

    # Development tools
    sqlx-cli

    # Other utilities
    jq
    curl
    git
  ];

  # Environment variables
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  # SpatiaLite library path
  SPATIALITE_LIB_PATH = "${pkgs.libspatialite}/lib/mod_spatialite.so";

  # Database URLs for development
  DATABASE_URL = "sqlite://cache.db";

  shellHook = ''
    echo "🦀 Rust development environment loaded"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Rust:        $(rustc --version)"
    echo "Cargo:       $(cargo --version)"
    echo "SpatiaLite:  ${pkgs.libspatialite}/lib/mod_spatialite.so"
    echo ""
    echo "Quick start:"
    echo "  cargo build          - Build the project"
    echo "  cargo run --bin cache-cli -- seed - Populate database with sample data"
    echo "  cargo test           - Run tests"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Create data directory
    mkdir -p data/tiles

    export LD_LIBRARY_PATH="${pkgs.libspatialite}/lib:$LD_LIBRARY_PATH"
  '';
}
