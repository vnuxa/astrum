{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    expat
    fontconfig
    freetype
    freetype.dev
    libGL
    pkg-config
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    wayland
    libxkbcommon
    dbus
    lua5_4_compat
  ];

  LD_LIBRARY_PATH =
    builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
}
