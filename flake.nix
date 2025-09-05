{
  inputs = {
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
    rust-overlay,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      naersk-lib = pkgs.callPackage naersk {
        cargo = toolchain;
        rustc = toolchain;
        clippy = toolchain;
      };
      apple-sdk = pkgs.lib.optionals pkgs.stdenv.isDarwin
        [
          pkgs.apple-sdk
        ];
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
              libiconv
              openssl
              pre-commit
            ]
            ++ apple-sdk;
          # RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      formatter = pkgs.alejandra;
    });
}
