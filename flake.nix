{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          commonArgs = {
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            src = craneLib.cleanCargoSource ./.;
            nativeBuildInputs = with pkgs; [
              rustToolchain
              pkg-config
            ];
            buildInputs = with pkgs; [
              zlib
              cmake
              clang
              libclang
            ];
          };

          bin = craneLib.buildPackage (commonArgs // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          });
        in
        with pkgs;
        {
          packages = {
            inherit bin;
            default = bin;
          };
          devShells.default = mkShell {
            inputsFrom = [ bin ];
            RUST_BACKTRACE = 1;
          };
        }
      );
}
