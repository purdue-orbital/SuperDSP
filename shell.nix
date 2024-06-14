{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell{
	buildInputs = with pkgs; [
		gcc
	];

	nativeBuildInputs = with pkgs; [
		#pkg-config
	];
}
