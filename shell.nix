{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    gcc
    pkg-config
    libusb1
  ];

  shellHook = ''
    export PKG_CONFIG_PATH="${pkgs.libusb1.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
  '';
}