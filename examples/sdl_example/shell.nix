{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    SDL2
    SDL2_gfx
  ];

  PKG_CONFIG_PATH="${pkgs.SDL2}/lib:${pkgs.SDL2_gfx}/lib";
}
