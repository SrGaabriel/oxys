{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  },
}:

let
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
pkgs.mkShell {
  packages = with pkgs; [
    toolchain
    cmake
    zstd
    rocmPackages.clr
    rocmPackages.rocminfo
  ];
}
