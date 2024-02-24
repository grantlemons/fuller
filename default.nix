{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = with pkgs; [ 
    pkgs.rustup 
    pkgs.pkg-config
    pkgs.openssl
  ];

  postHook = ''
    ${pkgs.rustup}/bin/rustup toolchain install nightly
  '';
}

