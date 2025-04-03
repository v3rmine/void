{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, fenix, advisory-db }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          system = "x86_64-linux";
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-musl" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;

          CARGO_BUILD_PACKAGE = "sis_server";
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";

          buildInputs = with pkgs; [
            # Add extra build inputs here, etc.
            # openssl
          ];

          nativeBuildInputs = with pkgs; [
            # Add extra native build inputs here, etc.
            pkg-config
            clang_14
            mold
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          # Additional arguments specific to this derivation can be added here.
          # Be warned that using `//` will not do a deep copy of nested
          # structures
          pname = "sis-deps";
        });

        sisServer = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          
          LOG_DIRECTORY = "/var/log/sis-server";
        });

        sisDoc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
        });

        checkClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        checkTests = craneLib.cargoNextest (commonArgs // {
          inherit cargoArtifacts;
        });

        checkFmt = craneLib.cargoFmt (commonArgs // {
          inherit cargoArtifacts;
        });

        checkAudit = craneLib.cargoAudit (commonArgs // {
          inherit cargoArtifacts advisory-db;
        });

        dockerSisServer = pkgs.dockerTools.buildImage {
          name = "sis-server";
          tag = "latest";
          copyToRoot = [ sisServer ];
          config = {
            Env = [
              "HOST=0.0.0.0"
            ];
            EntryPoint = [ "${sisServer}/bin/sis-server" ];
          };
        };
      in { 
        packages.default = sisServer;
        apps.default = {
          type = "app";
          program = "${sisServer}/bin/sis-server";
        };

        packages.docker = dockerSisServer;

        checks = {
          inherit
            # Build the crate as part of `nix flake check` for convenience
            sisServer
            checkFmt
            checkClippy
            checkTests;
        };
      });
}

