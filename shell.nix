{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec {
  SHELL_NAME = "RS";
  buildInputs = [ pkgs.cargo pkgs.rustc pkgs.openssl pkgs.pkg-config ];
}

