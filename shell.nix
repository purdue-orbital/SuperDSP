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
        pkgs.libbladeRF
    ]}";
}
