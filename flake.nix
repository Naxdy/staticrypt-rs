{
  description = "Rust crate for static encryption.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    crane.url = "github:ipetkov/crane";

    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ fenix.overlays.default ];
            };

            rustToolchain = pkgs.fenix.stable.withComponents [
              "cargo"
              "rustc"
              "rustfmt"
              "rust-analyzer"
              "clippy"
            ];

            craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

            craneArgs = {
              src = craneLib.cleanCargoSource ./.;
              strictDeps = true;

              env = {
                RUST_BACKTRACE = "1";
                STATICRYPT_SEED = "01234567890123456789012345678901";
                RUSTFLAGS = "-Dwarnings";
                RUSTDOCFLAGS = "-Dwarnings";
              };
            };

            cargoArtifacts = craneLib.buildDepsOnly craneArgs;
          in
          f {
            inherit
              pkgs
              rustToolchain
              craneLib
              cargoArtifacts
              craneArgs
              ;
          }
        );
    in
    {
      devShells = forEachSupportedSystem (
        { pkgs, rustToolchain, ... }:
        {
          default = pkgs.mkShell {
            nativeBuildInputs = [
              rustToolchain
            ];

            STATICRYPT_SEED = "01234567890123456789012345678901";
          };
        }
      );

      checks = forEachSupportedSystem (
        {
          pkgs,
          craneLib,
          cargoArtifacts,
          craneArgs,
          ...
        }:
        {
          cargoTest = craneLib.cargoTest (
            craneArgs
            // {
              inherit cargoArtifacts;

              postUnpack = ''
                cp ${./testfile.txt} ./source/testfile.txt
              '';
            }
          );
        }
      );
    };
}
