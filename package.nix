{ rustPlatform,
  stdenv,
  AppKit,
  CoreFoundation,
  CoreServices,
  Foundation,
  Security
}:
let
  optionals = stdenv.optionals;
  isDarwin = stdenv.isDarwin;
#let
#  pkgs = import <nixpkgs> { };
#  rustPlatform = pkgs.rustPlatform;
#  optionals = pkgs.lib.optionals;
#  isDarwin = pkgs.stdenv.isDarwin;
#  AppKit = pkgs.darwin.apple_sdk.frameworks.AppKit;
#  CoreFoundation = pkgs.darwin.apple_sdk.frameworks.CoreFoundation;
#  CoreServices = pkgs.darwin.apple_sdk.frameworks.CoreServices;
#  Foundation = pkgs.darwin.apple_sdk.frameworks.Foundation;
#  Security = pkgs.darwin.apple_sdk.frameworks.Security;
in
rustPlatform.buildRustPackage rec {
  pname = "jira-tools";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256-K54xldM+1hUn84CHCr7CL23bjZdx7tWAPKDYfamOlZI=";

  buildInputs = optionals isDarwin [
    AppKit
    CoreFoundation
    CoreServices
    Foundation
    Security
  ];
}
