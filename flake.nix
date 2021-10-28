{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, rust-overlay, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        devRustNightly = pkgs.rust-bin.nightly."2021-08-31".default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            boost
            clang
            cmake
            openssl
            pkg-config
            devRustNightly
          ];

          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          PROTOC = "${protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        };
      }
    );
}
