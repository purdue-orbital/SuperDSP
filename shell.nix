{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell{
	buildInputs = with pkgs; [
		gcc
		rustc
		cargo
		pkg-config
		fontconfig

        libxkbcommon
        libGL

        wayland

        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libX11

        git
        cmake

        libbladeRF
	];
	LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
        pkgs.rustc
        pkgs.cargo
        pkgs.libxkbcommon
        pkgs.libGL
        pkgs.wayland

        pkgs.xorg.libXcursor
        pkgs.xorg.libXrandr
        pkgs.xorg.libXi
        pkgs.xorg.libX11

        pkgs.libbladeRF
    ]}";
}
