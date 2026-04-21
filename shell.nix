{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.qemu
    pkgs.jq
    pkgs.pkgsCross.aarch64-multiplatform.buildPackages.gcc
    pkgs.pkgsCross.aarch64-multiplatform.buildPackages.binutils
    pkgs.lld
    pkgs.rustup
  ];

  CC = "aarch64-unknown-linux-gnu-gcc";
  AS = "aarch64-unknown-linux-gnu-as";

  shellHook = ''
    rustup target add aarch64-unknown-none
    alias make="make CC=aarch64-unknown-linux-gnu-gcc AS=aarch64-unknown-linux-gnu-as"
  '';
}
