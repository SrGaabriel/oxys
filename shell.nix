{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
    cmake
    zstd
    rocmPackages.clr
    rocmPackages.rocminfo
  ];
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
}
