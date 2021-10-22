let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs.latest.rustChannels.nightly; [
    cargo
    rust
    pkgs.rustfmt
  ];
  buildInputs = [
    pkgs.libiconv
    pkgs.pkgconf
  ];
}
