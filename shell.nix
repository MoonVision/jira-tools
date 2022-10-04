let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.cargo
    pkgs.pre-commit
    pkgs.rustfmt
  ];
  buildInputs = [
    pkgs.libiconv
    pkgs.pkgconf
  ] ++ (
    pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.Security
      pkgs.darwin.apple_sdk.frameworks.CoreServices
      pkgs.darwin.apple_sdk.frameworks.CoreFoundation
      pkgs.darwin.apple_sdk.frameworks.Foundation
      pkgs.darwin.apple_sdk.frameworks.AppKit
    ]
  );
}
