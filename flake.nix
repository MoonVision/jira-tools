{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-substituters = ["https://attic.internal.moonvision.io/moonvision-foss"];
    extra-trusted-public-keys = ["moonvision-foss:9nKTaMgGQ1M1+CYCftDNWQD39fESVxcp9o8GwRf1B3M="];
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      naersk-lib = pkgs.callPackage naersk {};
      apple-sdk = pkgs.lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          AppKit
          CoreFoundation
          CoreServices
          Foundation
          Security
          SystemConfiguration
        ]
      );
    in {
      defaultPackage = self.packages.${system}.jira-tools;
      packages = {
        default = self.packages.${system}.jira-tools;
        jira-tools = naersk-lib.buildPackage {
          src = ./.;
          buildInputs = with pkgs;
            [
              openssl
              pkg-config
            ]
            ++ apple-sdk;
        };
      };
      devShell = with pkgs;
        mkShell {
          buildInputs =
            [
              cargo
              libiconv
              openssl
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
            ]
            ++ apple-sdk;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      formatter = pkgs.alejandra;
    });
}
