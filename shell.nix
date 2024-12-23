let
  pkgs =
    import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc];
  };
in with pkgs;
mkShell {
  SHELL_NAME = "RS";
  buildInputs = [ cargo rustc openssl pkg-config jetbrains.rust-rover rust-toolchain ];
  RUST_BACKTRACE = 1;
}
