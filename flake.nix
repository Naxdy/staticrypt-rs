{
  description = "Rust crate for static encryption.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    crane.url = "github:ipetkov/crane";

    fenix.url = "github:nix-community/fenix";

    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      treefmt-nix,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
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
                # used in doc test
                MY_SECRET_VAR = "super secret env";
                RUSTFLAGS = "-Dwarnings";
                RUSTDOCFLAGS = "-Dwarnings";
              };
            };

            cargoArtifacts = craneLib.buildDepsOnly craneArgs;

            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

            treefmtEval = treefmt-nix.lib.evalModule pkgs (
              import ./treefmt.nix { inherit rustToolchain cargoToml; }
            );

            treefmt = treefmtEval.config.build.wrapper;
          in
          f {
            inherit
              cargoArtifacts
              craneArgs
              craneLib
              pkgs
              rustToolchain
              treefmt
              treefmtEval
              ;
          }
        );
    in
    {
      formatter = forEachSupportedSystem ({ treefmt, ... }: treefmt);

      devShells = forEachSupportedSystem (
        {
          pkgs,
          rustToolchain,
          treefmt,
          ...
        }:
        {
          default = pkgs.mkShell {
            nativeBuildInputs = [
              rustToolchain
              treefmt
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
          treefmtEval,
          ...
        }:
        {
          treefmt = treefmtEval.config.build.check self;

          cargoDoc = craneLib.cargoDoc (craneArgs // { inherit cargoArtifacts; });

          cargoTest = craneLib.cargoTest (
            craneArgs
            // {
              inherit cargoArtifacts;

              postUnpack = ''
                cp ${./testfile.txt} ./source/testfile.txt
              '';
            }
          );

          cargoClippy = craneLib.cargoClippy (craneArgs // { inherit cargoArtifacts; });
        }
      );
    };
}
