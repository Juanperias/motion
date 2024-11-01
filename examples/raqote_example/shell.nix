

{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    alsa-lib
    dbus
    fontconfig
    libGL
    libpulseaudio
    libxkbcommon
    makeWrapper
    mesa
    patchelf
    speechd
    udev
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXext
    xorg.libXfixes
    xorg.libXi
    xorg.libXinerama
    xorg.libXrandr
    xorg.libXrender
  ];

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.alsa-lib}/lib:${pkgs.dbus}/lib:${pkgs.fontconfig}/lib:${pkgs.libGL}/lib:${pkgs.libpulseaudio}/lib:${pkgs.libxkbcommon}/lib:${pkgs.mesa}/lib:${pkgs.vulkan-loader}/lib:${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXext}/lib:${pkgs.xorg.libXfixes}/lib:${pkgs.xorg.libXi}/lib:${pkgs.xorg.libXinerama}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.xorg.libXrender}/lib:$LD_LIBRARY_PATH

  '';
}

